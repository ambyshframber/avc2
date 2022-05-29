use wrapping_arithmetic::wrappit;

use crate::dev::{DevicePage, DevSpec};
use crate::utils::Avc2Error;

const MEM_SIZE: u16 = 0xFF00;

pub struct Mem {
    main: [u8; MEM_SIZE as usize],
    devices: DevicePage
}

impl Mem {
    /// program start is 0x0300
    pub fn new_from_rom(rom: &[u8], devs: Vec<DevSpec>) -> Result<Mem, Avc2Error> {
        let mut main = [0; MEM_SIZE as usize];
        for (i, byte) in rom.iter().enumerate() {
            main[i + 0x0300] = *byte
        }
        Ok(Mem {
            main,
            devices: DevicePage::new(devs)?
        })
    }

    pub fn get(&mut self, idx: u16) -> u8 {
        if idx < MEM_SIZE {
            self.main[idx as usize]
        }
        else { // devices
            self.devices.read(idx as u8)
        }
    }
    #[wrappit]
    pub fn set(&mut self, idx: u16, val: u8) {
        //eprintln!("CELL {:04x} SET TO {:02x}\r", idx, val);
        if idx < MEM_SIZE {
            self.main[idx as usize] = val
        }
        else { // devices
            match self.devices.write(idx as u8, val) {
                Some(DmaRequest::ToDev{addr, len}) => {
                    let mut ret = Vec::new();
                    for i in 0..len {
                        ret.push(self.get(i + addr))
                    }
                    self.devices.dma_callback(ret)
                }
                Some(DmaRequest::ToMem{addr, data}) => {
                    //eprintln!("DMACTL TOMEM\r");
                    for (i, b) in data.iter().enumerate() {
                        self.set((i as u16)+ addr, *b)
                    }
                }
                _ => {}
            }
        }
    }

    #[wrappit]
    pub fn get_16(&mut self, idx: u16) -> u16 {
        let hb = self.get(idx);
        let lb = self.get(idx + 1);
        u16::from_be_bytes([hb, lb])
    }
    #[wrappit]
    pub fn set_16(&mut self, idx: u16, val: u16) {
        let [hb, lb] = val.to_be_bytes();
        self.set(idx, hb);
        self.set(idx + 1, lb)
    }
}

pub enum DmaRequest {
    ToMem{addr: u16, data: Vec<u8>},
    ToDev{addr: u16, len: u16}
}
