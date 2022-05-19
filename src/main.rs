mod utils;
mod memory;
mod processor;
mod dev;

use processor::Processor;
use std::env::args;
use std::fs::read;

fn main() {
    let a: Vec<String> = args().collect();
    let rom = if a.len() != 2 {
        panic!()
    }
    else {
        read(&a[1]).unwrap()
    };
    if &rom[..4] != &[0x41, 0x56, 0x43, 0x00] {
        panic!()
    }
    let mut p = Processor::new(&rom[4..]);
    loop {
        p.execute_once()
    }
}
