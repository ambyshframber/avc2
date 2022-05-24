use super::{Device, WriteResponse};
use avdrive::{Avd, AvdError};
use std::path::{PathBuf, Path};
use crate::utils::*;

/// DRIVE DEVICE
/// 
/// idx typ use
/// 0   r   devid, returns 2
/// 2   w   block hb
/// 3   w   block lb
/// 4   w   page
/// 8   w   read, drive -> mem
/// 9   w   write, mem -> drive

pub struct Drive {
    drive: Avd,
    archive_path: PathBuf,
    block: u16,
    page: u8
}
impl Drive {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Drive, AvdError> {
        let drive = Avd::from_host_drive(&path)?;
        let archive_path = PathBuf::from(path.as_ref());
        Ok(Drive {
            drive, archive_path,
            block: 0,
            page: 0
        })
    }
}
impl Device for Drive {
    fn read(&mut self, addr: u8) -> u8 {
        match addr {
            0 => 2,
            _ => 0
        }
    }
    fn write(&mut self, addr: u8, val: u8) -> WriteResponse {
        //eprintln!("DRVCTL {:02x} {:02x}\r", addr, val);
        match addr {
            2 => self.block = set_hb(self.block, val),
            3 => self.block = set_lb(self.block, val),
            4 => self.page = val,
            8 => {
                let data = self.drive.get_block(self.block).unwrap_or([0; 256]).to_vec();
                let addr = u16::from_be_bytes([self.page, 0]);
                //eprintln!("DRVCTL SEND DMA\r");
                return WriteResponse::DmaToMem {
                    addr, data
                }
            }
            9 => {
                let addr = u16::from_be_bytes([self.page, 0]);
                return WriteResponse::DmaToDev {
                    addr, len: 256
                }
            }
            _ => {}
        }
        WriteResponse::None
    }
    fn shutdown(&mut self) {
        let _ = self.drive.save(&self.archive_path);
    }
    fn dma_callback(&mut self, data: Vec<u8>) {
        assert!(data.len() == 256);
        self.drive.set_block(self.block, &data.try_into().unwrap())
    }
}
