use std::fmt;

pub type Screen = [[bool; 64]; 32];

pub struct Display {
    pub screen: Screen,
}

const ZERO_DISPLAY: Screen = [[false; 64]; 32];

impl Display {
    pub fn new() -> Self {
        Display {
            screen: ZERO_DISPLAY,
        }
    }

    pub fn clear(&mut self) {
        self.screen = ZERO_DISPLAY;
    }

    pub fn draw_sprite(
        &mut self,
        _collision: &mut u8,
        start_col: u8,
        start_row: u8,
        sprite: &[u8],
    ) {
        let mut row: usize = start_row as usize;
        for &byte in sprite {
            let mut col: usize = start_col as usize;
            let mut mask = 0x80;
            while mask > 0 {
                self.screen[row][col] ^= byte & mask > 0;
                col += 1;
                col %= 64;
                mask /= 2;
            }
            row += 1;
            row %= 32;
        }
    }
}

impl fmt::Debug for Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use std::fmt::Write;

        for row in &self.screen {
            for &cell in row.iter() {
                f.write_char(if cell { '█' } else { ' ' })?;
            }
            f.write_char('\n')?;
        }

        Ok(())
    }
}
