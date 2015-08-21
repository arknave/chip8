use std::env;
use std::fs::File;
use std::io;

pub mod cpu;
pub mod mem;

fn read_from_file(path: &str, vec: &mut Vec<u8>) -> io::Result<usize> {
    use std::io::Read;
    let mut f = try!(File::open(path));
    f.read_to_end(vec)
}

/**
 * Necessary steps:
 * 1. read in files
 * 2. parse bytecode
 * 3. execute bytecode
 * 4. handle display & sound
 */
fn main() {
    let args = env::args();
    if args.len() != 2 {
        panic!("Need a filename passed in");
    }

    let mut cpu = cpu::Cpu::new();

    let file_path = args.last().expect("Need a valid filename");
    let mut rom = vec!();
    read_from_file(&file_path, &mut rom).unwrap();
    cpu.run(rom);
}
