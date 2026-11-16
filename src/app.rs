use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

use crate::model::Project;

/// UI state: the project list, which entry is selected and whether to quit.
/// Holds no terminal handle, so every transition is testable as plain data.
#[derive(Debug, Default)]
pub struct App {
    projects: Vec<Project>,
    selected: usize,
    exit: bool,
}

impl App {
    pub fn new(projects: Vec<Project>) -> Self {
        Self {
            projects,
            selected: 0,
            exit: false,
        }
    }

    pub fn projects(&self) -> &[Project] {
        &self.projects
    }

    /// Index of the selected project, `None` when the list is empty.
    pub fn selected(&self) -> Option<usize> {
        if self.projects.is_empty() {
            None
        } else {
            Some(self.selected)
        }
    }

    pub fn selected_project(&self) -> Option<&Project> {
        self.projects.get(self.selected)
    }

    pub fn should_exit(&self) -> bool {
        self.exit
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) {
        if key_event.kind != KeyEventKind::Press {
            return;
        }
        match key_event.code {
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Char('j') | KeyCode::Down => self.select_next(),
            KeyCode::Char('k') | KeyCode::Up => self.select_previous(),
            _ => {}
        }
    }

    /// Moves down one entry, clamped at the last project.
    pub fn select_next(&mut self) {
        if self.selected + 1 < self.projects.len() {
            self.selected += 1;
        }
    }

    /// Moves up one entry, clamped at the first project.
    pub fn select_previous(&mut self) {
        self.selected = self.selected.saturating_sub(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn app_with(names: &[&str]) -> App {
        let projects = names
            .iter()
            .map(|name| Project {
                name: name.to_string(),
                path: PathBuf::from(name),
            })
            .collect();
        App::new(projects)
    }

    fn selected_name(app: &App) -> Option<&str> {
        app.selected_project().map(|project| project.name.as_str())
    }

    #[test]
    fn starts_on_first_project() {
        let app = app_with(&["alpha", "beta"]);
        assert_eq!(app.selected(), Some(0));
        assert_eq!(selected_name(&app), Some("alpha"));
        assert!(!app.should_exit());
    }

    #[test]
    fn j_k_and_arrows_move_selection() {
        let mut app = app_with(&["alpha", "beta", "zeta"]);

        app.handle_key_event(KeyCode::Char('j').into());
        assert_eq!(selected_name(&app), Some("beta"));

        app.handle_key_event(KeyCode::Down.into());
        assert_eq!(selected_name(&app), Some("zeta"));

        app.handle_key_event(KeyCode::Char('k').into());
        assert_eq!(selected_name(&app), Some("beta"));

        app.handle_key_event(KeyCode::Up.into());
        assert_eq!(selected_name(&app), Some("alpha"));
    }

    #[test]
    fn selection_clamps_at_both_ends() {
        let mut app = app_with(&["alpha", "beta"]);

        app.select_previous();
        assert_eq!(app.selected(), Some(0), "clamped at top");

        app.select_next();
        app.select_next();
        app.select_next();
        assert_eq!(app.selected(), Some(1), "clamped at bottom");
    }

    #[test]
    fn empty_list_has_no_selection() {
        let mut app = app_with(&[]);
        app.select_next();
        app.select_previous();
        assert_eq!(app.selected(), None);
        assert!(app.selected_project().is_none());
    }

    #[test]
    fn q_quits_and_release_events_are_ignored() {
        let mut app = app_with(&["alpha", "beta"]);

        let mut release = KeyEvent::from(KeyCode::Char('j'));
        release.kind = KeyEventKind::Release;
        app.handle_key_event(release);
        assert_eq!(app.selected(), Some(0), "release does not move");

        app.handle_key_event(KeyCode::Char('q').into());
        assert!(app.should_exit());
    }
}
