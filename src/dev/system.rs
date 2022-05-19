use std::fs::File;
use std::io::{Read, Write, stdout, stderr, Stdout, Stderr};
use super::Device;

/// SYSTEM DEVICE
/// 
/// idx type use  
/// 0   r    devid, returns 1 
/// 1   w    wait, wait x ms
/// 2   r    random, random in range [0, 256)
/// 8   r    stdin
/// 9   w    stdout
/// a   w    stderr
/// b   r    buf, returns amount of bytes in buffer (unsigned), 255 if over 255
/// f   w    halt
/// 
pub struct System {
    lfsr: u16,
    stdout: Stdout,
    stderr: Stderr
}

impl System {
    pub fn new() -> Self { 
        let mut random = File::open("/dev/urandom").unwrap();
        let mut lfsr = [0u8; 2];
        random.read_exact(&mut lfsr).unwrap();
        let lfsr = u16::from_be_bytes(lfsr);
        System {
            lfsr,
            stdout: stdout(),
            stderr: stderr()
        }
    }
    fn advance_lfsr(&mut self) {
        self.lfsr ^= self.lfsr >> 7;
        self.lfsr ^= self.lfsr << 9;
        self.lfsr ^= self.lfsr >> 13;
    }
}

impl Device for System {
    fn write(&mut self, addr: u8, val: u8) {
        self.advance_lfsr();
        match addr {
            9 => {
                self.stdout.write(&[val]).unwrap();
                self.stdout.flush().unwrap();
            }
            0xa => {
                self.stderr.write(&[val]).unwrap();
                self.stderr.flush().unwrap();
            }
            0xf => std::process::exit(0), // halt (lol)
            _ => {}
        }
    }
    fn read(&mut self, addr: u8) -> u8 {
        self.advance_lfsr();
        match addr {
            0 => 1, // devid
            2 => self.lfsr.to_be_bytes()[0], // random
            _ => 0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_write() {
        let mut sys = System::new();
        println!("{}", sys.read(0x2));
        sys.write(0x9, 0x69);
        sys.write(0x9, 0xa);
        println!("{}", sys.read(0x2));
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
