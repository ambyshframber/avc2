mod utils;
mod memory;
mod processor;
mod dev;

use processor::Processor;
use std::fs::read;
use clap::{Arg, Command};
use dev::DevSpec;

fn main() {
    let matches = Command::new("avc2")
        .version("0.1.0")
        .arg(Arg::new("ROM").required(true).help("the rom file to execute"))
        .arg(Arg::new("DEVICE")
            .short('d')
            .required(false)
            .takes_value(true)
            .multiple_occurrences(true)
            .help("a device to add. device formats are detailed in the readme.")
        )
        .get_matches()
    ;
    let devs = if let Some(v) = matches.values_of("DEVICE") {
        v.map(|d| DevSpec::from_str(d)).collect()
    }
    else {
        Ok(Vec::new())
    }.unwrap();
    let rom = read(matches.value_of("ROM").unwrap()).unwrap();

    if &rom[..4] != &[0x41, 0x56, 0x43, 0x00] {
        panic!("bad signature!")
    }
    let mut p = Processor::new(&rom[4..], devs).unwrap();
    loop {
        p.execute_once()
    }
}
