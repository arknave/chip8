// set up the event loop
use std::{
    error::Error,
    io::{self, Write},
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event as CEvent, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};

use tui::{backend::CrosstermBackend, Terminal};

pub mod app;
mod cpu;
mod display;
pub mod ui;

use crate::app::App;

enum Event<I> {
    Input(I),
    Tick,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = std::env::args();
    if args.len() != 2 {
        panic!("Need a filename passed in");
    }

    let file_path = args.last().expect("Need a valid filename");
    let rom: Vec<u8> = std::fs::read(file_path)?;

    crossterm::terminal::enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;

    let (tx, rx) = std::sync::mpsc::channel();

    let tick_rate = std::time::Duration::from_millis(25);

    let mut app = App::new(&rom);

    // Timer thread
    std::thread::spawn(move || {
        let mut last_tick = std::time::Instant::now();
        loop {
            // poll for tick rate. If no events, send tick event
            if event::poll(tick_rate - last_tick.elapsed()).unwrap() {
                if let CEvent::Key(key) = event::read().unwrap() {
                    tx.send(Event::Input(key)).unwrap();
                }
            }

            if last_tick.elapsed() >= tick_rate {
                tx.send(Event::Tick).unwrap();
                last_tick = std::time::Instant::now();
            }
        }
    });

    terminal.clear()?;

    loop {
        terminal.draw(|f| ui::draw(f, &app))?;

        let mut should_quit = false;
        match rx.recv()? {
            Event::Input(event) => match event.code {
                KeyCode::Char('q') => {
                    crossterm::terminal::disable_raw_mode()?;
                    execute!(
                        terminal.backend_mut(),
                        LeaveAlternateScreen,
                        DisableMouseCapture,
                    )?;
                    terminal.show_cursor()?;
                    break;
                }
                KeyCode::Char(c) => {
                    app.on_key(c);
                }
                _ => {
                    should_quit = true;
                }
            },
            Event::Tick => {
                app.on_tick();
            }
        }

        if should_quit {
            break;
        }
    }

    Ok(())
}
