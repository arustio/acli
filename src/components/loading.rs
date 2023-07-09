use std::io;
use tui::{backend::TermionBackend, Terminal, widgets::{Block, Borders, Paragraph}, layout::{Direction, Layout, Constraint, Alignment}};
use termion::{raw::IntoRawMode, color::Fg, style::Bold};

pub struct Loading {
    text: String,
}

impl Loading {
    pub fn new() -> Self {
        Self {
            text: "Loading...".to_string(),
        }
    }

    pub fn render(&self) -> Result<(), io::Error> {
        let stdout = io::stdout().into_raw_mode()?;
        let backend = TermionBackend::new(stdout);
        let mut terminal: Terminal<TermionBackend<termion::raw::RawTerminal<io::Stdout>>> = Terminal::new(backend)?;
        let mut frame = terminal.get_frame();
                
        let fan_icon = format!("{}{}", Fg(termion::color::Red), "⠋");
        let fan_text = Paragraph::new(fan_icon)
            .block(Block::default().borders(Borders::ALL));
    
        // let loading_text = Paragraph::new("加载中...")
        //     .block(Block::default().borders(Borders::ALL))
        //     .alignment(Alignment::Center);
    
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Percentage(100),
            ])
            .split(frame.size());
    
        frame.render_widget(fan_text, layout[0]);
        
        terminal.flush()?;
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loading() {
        let loading = Loading::new();
        assert!(loading.render().is_ok());
    }
}
