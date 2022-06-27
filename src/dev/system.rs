#[allow(unused_imports)]
use std::io::{Read, Write, stdout, stderr, Stdout, Stderr, stdin, Stdin};
use super::{Device, WriteResponse};
//use rand::{thread_rng, Rng};
use std::time::SystemTime;
use termion::{AsyncReader, async_stdin, /*raw::{IntoRawMode, RawTerminal}*/};
use std::thread::sleep;
use std::time::Duration;

/// SYSTEM DEVICE
/// 
/// idx typ use  
/// 0   r   devid, returns 1 
/// 1   w   wait, wait x ms
/// 2   r   random, random in range [0, 256)
/// 8   r   stdin
/// 9   w   stdout
/// a   w   stderr
/// b   r   buf, returns amount of bytes in buffer (unsigned), 255 if over 255
/// f   w   halt
/// 
pub struct System {
    lfsr: u16,
    stdout: Stdout,
    stderr: Stderr,
    stdin: AsyncReader,
    buf: Vec<u8>
}

impl System {
    pub fn new() -> Self { 
        let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
        let mut lfsr = time.as_millis() as u16;
        if lfsr == 0 { lfsr = 1 }
        System {
            lfsr,
            stdout: stdout(),
            stderr: stderr(),
            stdin: async_stdin(),
            buf: Vec::new()
        }
    }
    fn advance_lfsr(&mut self) {
        self.lfsr ^= self.lfsr >> 7;
        self.lfsr ^= self.lfsr << 9;
        self.lfsr ^= self.lfsr >> 13;
        //println!("{}", self.lfsr)
    }
    fn update_buf(&mut self) {
        self.stdin.read_to_end(&mut self.buf).unwrap();
    }
}

impl Device for System {
    fn write(&mut self, addr: u8, val: u8) -> WriteResponse {
        self.advance_lfsr();
        match addr {
            1 => {
                let d = Duration::from_millis(val as u64);
                sleep(d)
            }
            9 => {
                self.stdout.write(&[val]).unwrap();
                self.stdout.flush().unwrap();
            }
            0xa => {
                self.stderr.write(&[val]).unwrap();
                self.stderr.flush().unwrap();
            }
            _ => {}
        }
        if addr == 0x0f {
            WriteResponse::Shutdown(val)
        }
        else {
            WriteResponse::None
        }
    }
    fn read(&mut self, addr: u8) -> u8 {
        self.advance_lfsr();
        match addr {
            0 => 1, // devid
            2 => self.lfsr.to_be_bytes()[0], // random
            8 => {
                self.update_buf();
                if self.buf.len() == 0 {
                    0
                }
                else {
                    self.buf.remove(0)
                }
            }
            0xb => {
                self.update_buf();
                if self.buf.len() > 255 {
                    255
                }
                else {
                    self.buf.len() as u8
                }
            }
            _ => 0
        }
    }
    fn shutdown(&mut self) {
        //self.stdout.suspend_raw_mode().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_write() {
    }
    #[test]
    fn test_random() {
        let mut sys = System::new();
        let mut period = 0;
        let init = sys.read(0x2);
        loop {
            if sys.read(0x2) == init {
                break
            }
            period += 1
        }
        println!("{}", period)
    }
}
