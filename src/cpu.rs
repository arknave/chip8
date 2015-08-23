use std::thread;

use mem::Memory;
use display::Display;
use prng::Prng;

static FONTS: &'static [u8] = &[
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0x20, 0x60, 0x20, 0x20, 0x70, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct Cpu {
    memory: Memory, 
    display: Display,
    prng: Prng,

    registers: [u8; 16], // V1, V2, ..., VF
    register_index: u16, // VI
    pc: u16, // program counter

    stack: [u16; 16],
    sp: u8, // stack pointer
    // timers
    delay: u8,
    sound: u8,
}

impl Cpu {
    pub fn new() -> Self {
        let mut memory = Memory::new();
        Cpu::set_digits(&mut memory);

        Cpu { 
            memory: memory,
            display: Display::new(),
            prng: Prng::new(),

            registers: [0; 16],
            register_index: 0,
            pc: 0x200,

            stack: [0; 16],
            sp: 0,

            delay: 0,
            sound: 0,
        }
    }

    fn set_digits(memory: &mut Memory) {
        let mut offset = 0x50;
        // read from fonts into memory
        for &byte in FONTS {
            memory[offset] = byte;
            offset += 1;
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
        let opcode = ((self.memory[self.pc] as u16) << 8) | (self.memory[self.pc + 1] as u16);
        //println!("{:04x}: {:04x}", self.pc, opcode);
        (opcode != 0x1000 | self.pc) || (opcode != 0x0000)
    }

    fn read_register(&self, reg_index: &u8) -> u8 {
        self.registers[(*reg_index) as usize] 
    }

    fn register_equal(&self, reg_index: &u8, value: &u8) -> bool {
        self.read_register(reg_index) == *value
    }

    fn load_register(&mut self, reg_index: &u8, value: &u8) {
        self.registers[(*reg_index) as usize] = *value;
    }

    fn load_register_index(&mut self, value: &u16) {
        self.register_index = *value;
    }

    // TODO: Oh god get the random crate pls
    fn random_byte(&mut self) -> u8 {
        let rand = self.prng.next();
        rand as u8
    }

    fn random_register(&mut self, reg_index: &u8, and_with: &u8) {
        let random = self.random_byte() & (*and_with);
        self.load_register(reg_index, &random);
    }

    /**
     * Display a sprite. The sprite begins at (Vx, Vy), and is sprite_size lines tall.
     * Read the sprite in from memory from VI
     */
    fn draw_sprite(&mut self, reg_x: &u8, reg_y: &u8, sprite_size: &u8) {
        let vx = self.read_register(reg_x);
        let vy = self.read_register(reg_y);

        let sprite = self.memory.slice(&self.register_index, &(*sprite_size as u16));
        self.display.draw_sprite(&mut self.registers[15], &vx, &vy, &sprite);
        self.display.print();
    }

    /**
     * Execute a single instruction.
     */
    fn step(&mut self) {
        // read in 2 bytes
        let opcode: u16 = ((self.memory[self.pc] as u16) << 8) | (self.memory[self.pc + 1] as u16);
        let mut advance_pc = true;

        if opcode == 0x0000 {
            return;
        }

        let nnn: u16 =   opcode & 0x0FFF;
        let  kk:  u8 =  (opcode & 0x00FF) as u8;
        let   x:  u8 = ((opcode & 0x0F00) >> 8) as u8;
        let   y:  u8 = ((opcode & 0x00F0) >> 4) as u8;
        let   n:  u8 =  (opcode & 0x000F) as u8;

        match (opcode & 0xF000) {
            0x0000 => match kk {
                0xE0 => self.display.clear(),
                0xEE => {
                    self.sp -= 1;
                    self.pc = self.stack[(self.sp as usize)];
                },
                _ => self.unimplemented(opcode),
            },
            0x1000 => {
                self.pc = nnn;
                advance_pc = false;
            },
            0x2000 => {
                self.stack[(self.sp as usize)] = self.pc;
                self.sp += 1;
                self.pc = nnn;
                advance_pc = false;
            }
            0x3000 => {
                if self.register_equal(&x, &kk) {
                    advance_pc = false;
                    self.pc += 4;
                }
            },
            0x4000 => {
                if !self.register_equal(&x, &kk) {
                    advance_pc = false;
                    self.pc += 4;
                }
            },
            0x5000 => {
                let vy = self.read_register(&y);
                if self.register_equal(&x, &vy) {
                    advance_pc = false;
                    self.pc += 4;
                }
            }
            0x6000 => self.load_register(&x, &kk),
            0x7000 => {
                let new = self.read_register(&x).wrapping_add(kk);
                self.load_register(&x, &new);
            },
            0x8000 => match n {
                0x0 => {
                    let vy = self.read_register(&y);
                    self.load_register(&x, &vy); 
                },
                0x2 => {
                    let new = self.read_register(&x) & self.read_register(&y);
                    self.load_register(&x, &new);
                },
                0x4 => { 
                    // TODO: Figure out a better way to do carry detection
                    let big_new = (self.read_register(&x) as u16) + (self.read_register(&y) as u16);
                    let new = big_new as u8;
                    self.load_register(&x, &new); 
                    self.registers[15] = if big_new > 255 { 1 } else { 0 };
                },
                0x6 => {
                    let old = self.read_register(&x);
                    self.registers[15] = old & 1;
                    let new = old / 2;
                    self.load_register(&x, &new);
                },
                0xE => {
                    let old = self.read_register(&x);
                    self.registers[15] = (old & 0x80 > 0) as u8;
                    let new = old * 2;
                    self.load_register(&x, &new);
                },
                _ => self.unimplemented(opcode)
            },
            0xA000 => self.load_register_index(&nnn),
            0xC000 => self.random_register(&x, &kk),
            0xD000 => self.draw_sprite(&x, &y, &n),
            0xF000 => match kk {
                0x1E => {
                    let new = self.register_index + (self.read_register(&x) as u16);
                    self.load_register_index(&new);
                },
                0x55 => {
                    let index = self.register_index;
                    for reg in 0..(x + 1) {
                        self.memory[index + (reg as u16)] = self.registers[reg as usize];
                    }
                },
                0x65 => {
                    let index = self.register_index;
                    for reg in 0..(x + 1) {
                        self.registers[reg as usize] = self.memory[index + (reg as u16)];
                    }
                },
                _ => self.unimplemented(opcode),
            },
            _ => self.unimplemented(opcode),
        }

        if advance_pc {
            self.pc += 2;
        }
    }

    fn unimplemented(&self, opcode: u16) {
        panic!("Got unhandled opcode: {:04X}", opcode)
    }

    /**
     * Update the sound and delay timers.
     */
    fn update_timers(&mut self) {
        if self.delay > 0 {
            self.delay -= 1;
        }

        if self.sound > 0{
            self.sound -= 1;
        }   

        if self.sound > 0 {
            println!('\x07');
        }
    }

    /**
     * Update which keys are being held down.
     */
    fn update_inputs(&mut self) {

    }

    /**
     * Load and execute a rom.
     */
    pub fn run(&mut self, rom: Vec<u8>) {
        self.load_rom(rom);

        while self.running() {
            self.step();
            self.update_timers();
            self.update_inputs();

            thread::sleep_ms(10);
        }
    }
}
