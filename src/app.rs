// bridge between input, graphics, and logic

use crate::cpu::Cpu;

pub struct App {
    pub cpu: Cpu,
    running: bool,
}

impl App {
    pub fn new(rom: &Vec<u8>) -> Self {
        App {
            cpu: Cpu::new(rom),
            running: false,
        }
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            ' ' => self.cpu.step(),
            's' => self.running = !self.running,
            _ => {}
        }
    }

    pub fn on_tick(&mut self) {
        if !self.running {
            return;
        }

        self.cpu.step();
    }
}
