use system::System;
use drive::Drive;
use crate::utils::Avc2Error;
use crate::memory::DmaRequest;

mod system;
mod drive;

pub struct DevicePage {
    devs: [Option<Box<dyn Device>>; 16],
    last_dma_dev: u8
}

impl DevicePage {
    #[allow(unused_variables)]
    pub fn new(devs_to_use: Vec<DevSpec>) -> Result<DevicePage, Avc2Error> {
        let mut devs: [Option<Box<dyn Device>>; 16] = [0; 16].map(|_| None);
        devs[0] = Some(Box::new(System::new()));
        for spec in devs_to_use {
            if spec.loc == 0 {
                return Err(Avc2Error::DevInitError(String::from("dev 0 must be system")))
            }
            if spec.loc >= 16 {
                return Err(Avc2Error::DevInitError(String::from("device locations greater than 15 are invalid")))
            }
            match spec.id {
                1 => return Err(Avc2Error::DevInitError(String::from("only one system device may exist"))),
                2 => { // drive
                    let l = spec.options.len();
                    if l != 1 {
                        return Err(Avc2Error::DevInitError(format!("wrong # of options for drive device (expected 1, got {})", l)))
                    }
                    let d = Drive::new(spec.options[0])?;
                    devs[spec.loc] = Some(Box::new(d));
                }
                _ => return Err(Avc2Error::DevInitError(format!("unrecognised devid {}", spec.id)))
            }
        }

        Ok(DevicePage {
            devs,
            last_dma_dev: 0
        })
    }
    pub fn write(&mut self, addr: u8, val: u8) -> Option<DmaRequest> {
        let dev_idx = addr / 16;
        let addr = addr % 16;
        if let Some(d) = &mut self.devs[dev_idx as usize] {
            match d.write(addr, val) {
                WriteResponse::Shutdown(ecode) => {
                    for dev in &mut self.devs {
                        dev.as_mut().map(|d| d.shutdown());
                    }
                    println!();
                    std::process::exit(ecode as i32)
                }
                WriteResponse::DmaToMem{addr, data} => {
                    return Some(DmaRequest::ToMem{addr, data})
                }
                WriteResponse::DmaToDev{addr, len} => {
                    self.last_dma_dev = dev_idx;
                    return Some(DmaRequest::ToDev{addr, len})
                }
                _ => {}
            }
        }
        None
    }
    pub fn read(&mut self, addr: u8) -> u8 {
        let dev_idx = addr / 16;
        let addr = addr % 16;
        if let Some(d) = &mut self.devs[dev_idx as usize] {
            d.read(addr)
        }
        else {
            0
        }
    }
    pub fn dma_callback(&mut self, data: Vec<u8>) {
        if let Some(d) = &mut self.devs[self.last_dma_dev as usize] {
            d.dma_callback(data)
        }
    }
}

/// DEVICE PORT STRUCTURE
/// 
/// byte zero is the device id. this is a unique value in the range [2, 240]
/// used by no other device spec
/// 
/// all other bytes can be used however you like
trait Device {
    fn write(&mut self, addr: u8, val: u8) -> WriteResponse;
    fn read(&mut self, addr: u8) -> u8;
    fn shutdown(&mut self) {}
    fn dma_callback(&mut self, _data: Vec<u8>) {}
}

pub enum WriteResponse {
    None,
    Shutdown(u8),
    DmaToMem{addr: u16, data: Vec<u8>},
    DmaToDev{addr: u16, len: u16}
}
pub struct DevSpec<'a> {
    loc: usize,
    id: u8,
    options: Vec<&'a str>
}
impl DevSpec<'_> {
    pub fn new(loc: usize, id: u8, opt_string: &str) -> DevSpec {
        let options = opt_string.split(';').collect();
        DevSpec {
            loc, id, options
        }
    }
    pub fn from_str(s: &str) -> Result<DevSpec, Avc2Error> {
        let (locs, rest) = s.split_once(';').ok_or(Avc2Error::BadDevSpec(String::from(s)))?;
        let (ids, opts) = rest.split_once(';').ok_or(Avc2Error::BadDevSpec(String::from(s)))?;
        let loc = locs.parse().map_err(|_| Avc2Error::BadDevSpec(String::from(s)))?;
        let id = ids.parse().map_err(|_| Avc2Error::BadDevSpec(String::from(s)))?;
        Ok(DevSpec::new(loc, id, opts))
    }
}
