// TODO: make internals private
pub struct Display {
    display: [[bool; 64]; 32],
}

impl Display {
    pub fn new() -> Self {
        Display {
            display: [[false; 64]; 32],
        }
    }

    pub fn draw_sprite(&mut self, collision: &mut u8, start_col: u8, start_row: u8, sprite: &[u8]) {
        let mut row: usize = start_row as usize;
        for &byte in sprite {
            let mut col: usize = start_col as usize;
            let mut mask = 0x80;
            while mask > 0 {
                self.display[row][col] ^= byte & mask > 0;
                col += 1;
                col %= 64;
                mask /= 2;
            }
            row += 1;
            row %= 32;
        }
    }

    pub fn clear(&self) {
        // this incredibly portable command clears the screen in osx
        print!("\x1b\x5b\x48\x1b\x5b\x32\x4a");
    }

    pub fn print(&self) {
        self.clear();

        for row in &self.display {
            for &cell in row.iter() {
                print!("{}", if cell { '█' } else { ' ' });
            }
            println!("");
        }
    }
}
