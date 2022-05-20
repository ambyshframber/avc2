use system::System;

mod system;

pub struct DevicePage {
    devs: [Option<Box<dyn Device>>; 16]
}

impl DevicePage {
    #[allow(unused_variables)]
    pub fn new(devs_to_use: Vec<(u16, u8)>) -> DevicePage {
        let mut devs:[Option<Box<dyn Device>>; 16]  = [0; 16].map(|_| None);
        devs[0] = Some(Box::new(System::new()));

        DevicePage {
            devs
        }
    }
    pub fn write(&mut self, addr: u8, val: u8) {
        let dev_idx = addr / 16;
        let addr = addr % 16;
        if let Some(d) = &mut self.devs[dev_idx as usize] {
            d.write(addr, val)
        }
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
}

/// DEVICE PORT STRUCTURE
/// 
/// byte zero is the device id. this is a unique value in the range [2, 240]
/// used by no other device spec
/// 
/// all other bytes can be used however you like
trait Device {
    fn write(&mut self, addr: u8, val: u8);
    fn read(&mut self, addr: u8) -> u8;
}
