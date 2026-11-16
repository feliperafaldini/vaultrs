use ratatui::{
    Frame,
    style::{Modifier, Style},
    text::Line,
    widgets::{Block, List, ListState, Paragraph},
};

use crate::app::App;

/// Draws the project list with the selected entry highlighted.
pub fn draw(frame: &mut Frame, app: &App) {
    let block = Block::bordered()
        .title(" projects ")
        .title_bottom(Line::from(" j/k move · q quit ").centered());
    let area = frame.area();

    if app.projects().is_empty() {
        frame.render_widget(Paragraph::new("no projects").block(block), area);
        return;
    }

    let items = app.projects().iter().map(|project| project.name.as_str());
    let list = List::new(items)
        .block(block)
        .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
        .highlight_symbol("> ");
    let mut state = ListState::default().with_selected(app.selected());
    frame.render_stateful_widget(list, area, &mut state);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Project;
    use ratatui::{Terminal, backend::TestBackend};
    use std::path::PathBuf;

    fn render(app: &App) -> String {
        let mut terminal = Terminal::new(TestBackend::new(30, 6)).unwrap();
        terminal.draw(|frame| draw(frame, app)).unwrap();
        let buf = terminal.backend().buffer();
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
    fn renders_projects_with_selection_marker() {
        let projects = ["alpha", "beta"]
            .iter()
            .map(|name| Project {
                name: name.to_string(),
                path: PathBuf::from(name),
            })
            .collect();
        let mut app = App::new(projects);
        app.select_next();

        let text = render(&app);
        assert!(text.contains("projects"), "title missing:\n{text}");
        assert!(text.contains("  alpha"), "alpha missing:\n{text}");
        assert!(text.contains("> beta"), "selection missing:\n{text}");
        assert!(text.contains("q quit"), "help missing:\n{text}");
    }

    #[test]
    fn renders_placeholder_when_empty() {
        let text = render(&App::default());
        assert!(text.contains("no projects"), "placeholder missing:\n{text}");
    }
}
