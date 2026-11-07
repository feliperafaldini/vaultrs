use std::io;
use std::path::PathBuf;
use vaultrs::data::load_projects;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};

fn main() -> io::Result<()> {
    let dump = std::env::args().any(|arg| arg == "--dump");
    if dump {
        let root = vault_root()?;
        let projects = load_projects(&root)?;
        for project in projects {
            println!("{}", project.name);
        }
        return Ok(());
    }
    ratatui::run(|terminal| App::default().run(terminal))
}

fn vault_root() -> io::Result<PathBuf> {
    let home_dir = match std::env::var("HOME") {
        Ok(home_dir) => home_dir,
        Err(_) => {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "home directory not set.",
            ));
        }
    };
    let home_path = PathBuf::from(home_dir);
    let vault_dir = home_path.join("Documents/vault/projetos");
    Ok(vault_dir)
}

#[derive(Debug, Default)]
pub struct App {
    counter: u128,
    exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => self.decrement_counter(),
            KeyCode::Right => self.increment_counter(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn decrement_counter(&mut self) {
        self.counter -= 1;
    }

    fn increment_counter(&mut self) {
        self.counter += 1;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" vaultrs ".bold());
        let instructions = Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q>".blue().bold(),
        ]);

        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let counter_text = Text::from(vec![Line::from(vec![
            "Value: ".into(),
            self.counter.to_string().yellow(),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn buffer_text(buf: &Buffer) -> String {
        let mut text = String::new();
        for y in 0..buf.area.height {
            for x in 0..buf.area.width {
                text.push_str(buf[(x, y)].symbol());
            }
            text.push('\n');
        }
        text
    }

    #[test]
    fn render() {
        let app = App::default();
        let mut buf = Buffer::empty(Rect::new(0, 0, 50, 4));

        app.render(buf.area, &mut buf);

        let text = buffer_text(&buf);
        assert!(text.contains("vaultrs"), "title missing:\n{text}");
        assert!(text.contains("Value: 0"), "counter missing:\n{text}");
        assert!(
            text.contains("Decrement <Left>"),
            "left key missing:\n{text}"
        );
        assert!(
            text.contains("Increment <Right>"),
            "right key missing:\n{text}"
        );
        assert!(text.contains("Quit <Q>"), "quit key missing:\n{text}");
    }

    #[test]
    fn handle_key_event() {
        let mut app = App::default();
        app.handle_key_event(KeyCode::Right.into());
        assert_eq!(app.counter, 1);

        app.handle_key_event(KeyCode::Left.into());
        assert_eq!(app.counter, 0);

        app.handle_key_event(KeyCode::Char('q').into());
        assert!(app.exit);
    }
}
