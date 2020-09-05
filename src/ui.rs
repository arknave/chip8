use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::{app::App, cpu::Cpu};

fn parse_text(cpu: &Cpu) -> Vec<Spans> {
    cpu.display_bits()
        .iter()
        .map(|row| {
            Span::raw(
                row.iter()
                    .map(|b| if *b { '█' } else { ' ' })
                    .collect::<String>(),
            )
            .into()
        })
        .collect()
}

// Actually draw the terminal UI
pub fn draw<B: Backend>(f: &mut Frame<B>, app: &App) {
    let splits = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
        .split(f.size());

    let left_block = Block::default().title("Display").borders(Borders::ALL);

    let display = Paragraph::new(parse_text(&app.cpu))
        .block(left_block)
        .alignment(Alignment::Center);

    f.render_widget(display, splits[0]);

    let right_block = Block::default().title("Instructions").borders(Borders::ALL);

    let style = Style::default().add_modifier(Modifier::BOLD);

    let items: Vec<ListItem> = app
        .cpu
        .opcodes()
        .take(20)
        .enumerate()
        .map(|(i, opcode)| {
            let code = format!("{:04X}", opcode);
            ListItem::new(if i == 0 {
                Span::styled(code, style)
            } else {
                Span::raw(code)
            })
        })
        .collect();

    let instructions = List::new(items).block(right_block).highlight_symbol("> ");

    f.render_widget(instructions, splits[1]);
}
