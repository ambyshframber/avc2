use thiserror::Error;
use avdrive::AvdError;

#[derive(Error, Debug)]
pub enum Avc2Error {
    #[error("drive init error")]
    DriveError(#[from] AvdError),
    #[error("device init error: {0}")]
    DevInitError(String),
    #[error("bad device spec: {0}")]
    BadDevSpec(String)
}

pub fn set_hb(main: u16, hb: u8) -> u16 {
    let [_, lb] = main.to_be_bytes();
    u16::from_be_bytes([hb, lb])
}
pub fn set_lb(main: u16, lb: u8) -> u16 {
    let [hb, _] = main.to_be_bytes();
    u16::from_be_bytes([hb, lb])
}
