use std::env;
use std::fs::File;
use std::io;

pub mod cpu;
pub mod display;
pub mod mem;

extern crate rand;

/**
 * Necessary steps:
 * 1. read in files
 * 2. parse bytecode
 * 3. execute bytecode
 * 4. handle display & sound
 */
fn main() -> io::Result<()> {
    let args = env::args();
    if args.len() != 2 {
        panic!("Need a filename passed in");
    }

    let mut cpu = cpu::Cpu::new();

    let file_path = args.last().expect("Need a valid filename");
    let rom: Vec<u8> = std::fs::read(file_path)?;
    cpu.run(rom);

    Ok(())
}
