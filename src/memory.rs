use wrapping_arithmetic::wrappit;

use crate::dev::DevicePage;

const MEM_SIZE: u16 = 0xFF00;

pub struct Mem {
    main: [u8; MEM_SIZE as usize],
    devices: DevicePage
}

impl Mem {
    pub fn new() -> Mem {
        Self::new_from_rom(&[0; MEM_SIZE as usize])
    }
    /// program start is 0x0300
    pub fn new_from_rom(rom: &[u8]) -> Mem {
        let mut main = [0; MEM_SIZE as usize];
        for (i, byte) in rom.iter().enumerate() {
            main[i + 0x0300] = *byte
        }
        Mem {
            main,
            devices: DevicePage::new(Vec::new())
        }
    }

    pub fn get(&mut self, idx: u16) -> u8 {
        if idx < MEM_SIZE {
            self.main[idx as usize]
        }
        else { // devices
            self.devices.read(idx as u8)
        }
    }
    pub fn set(&mut self, idx: u16, val: u8) {
        //println!("CELL {:04x} SET TO {:02x}", idx, val);
        if idx < MEM_SIZE {
            self.main[idx as usize] = val
        }
        else { // devices
            self.devices.write(idx as u8, val)
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
