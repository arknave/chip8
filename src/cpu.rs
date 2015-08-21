use mem::Memory;

pub struct Cpu {
    memory: Memory, 
    registers: [u8; 16],
    register_index: u16,
    pc: u16,
    sp: u8,
}

impl Cpu {
    fn set_digits(mem: &mut Memory) {
        //TODO: Insert the digits in memory
    }

    pub fn new() -> Self {
        let mut memory = Memory::new();
        Cpu::set_digits(&mut memory);
        Cpu { 
            memory: memory,
            registers: [0; 16],
            register_index: 0,
            pc: 0x200,
            sp: 0,
        }
    }

    fn load_rom(&mut self, bytes: Vec<u8>) {
        let mut offset = 0x200;
        for byte in bytes {
            self.memory[offset] = byte;
            offset += 1;
        }
    }

    fn running(&self) -> bool {
        self.memory[self.pc] != 0
    }

    /**
     * Execute a single instruction.
     */
    fn step(&mut self) {
        // read in 2 bytes
        let opcode: u16 = ((self.memory[self.pc] as u16) << 8) | (self.memory[self.pc + 1] as u16);
        println!("Got opcode: {:04x}", opcode);
        self.pc += 2;
    }

    /**
     * Load and execute a rom.
     */
    pub fn run(&mut self, rom: Vec<u8>) {
        self.load_rom(rom);

        while self.running() {
            self.step();
        }
    }
}
