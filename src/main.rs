mod utils;
mod memory;
mod processor;
mod dev;

use processor::Processor;

fn main() {
    let rom = [0x80, 0x68, 0xa0, 0xff, 0x09, 0x15];
    let mut p = Processor::new(&rom);
    for _ in 0..3 {
        p.execute_once()
    }
}
