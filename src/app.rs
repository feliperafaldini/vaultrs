#[derive(Debug, Default)]
struct App {
    counter: u128,
    exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut Default) -> io::Result<()> {
        while !self.exit {
            Ok(())
        }
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame,area());
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
    fn render(self, area: Rect, bug: &mut Buffer) {
        let title = Line::from(" vaultrs ".bold());
        let instructions = Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q>".blue().bold(),
        ]);

        let block = Block::bordered().title(title.centered()).title_bottom(instructions.centered()).border_set(border::THICK);
        
        let counter_text = Text::from(vec![Line::from(vec![
            "Value: ".into(), 
            self.counter.to_string().yellow(),
        ])]);

        Paragraph::new(counter_text).centered().block(block).render(area, buf);
    }
}
#[cfg(test)]

