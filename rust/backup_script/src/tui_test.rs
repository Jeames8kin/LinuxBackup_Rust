#[allow(dead_code)]



use crate::io;
use termion::raw::IntoRawMode;
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Gauge},
    Terminal,
};

struct App {
    progress_bar1: u16,
}

impl App {
    fn new() -> App {
        App{
            progress_bar1: 0
        }   
    }

    fn update(&mut self) {
        self.progress_bar1 += 5;
        if self.progress_bar1 > 100 {
            self.progress_bar1 = 0;
        }
    }
}

pub fn main() -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    loop {    
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Percentage(25)
                    ]
                    .as_ref(),
                )
                .split(f.size());
            let gauge = Gauge::default()
                .block(Block::default()
                .title("Gauge1")
                .borders(Borders::ALL))
                .gauge_style(Style::default()
                .fg(Color::Yellow))
                .percent(app.progress_bar1);
            f.render_widget(gauge, chunks[0]);
       })?;

    }
    Ok(())
}