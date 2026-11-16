use std::io;
use std::path::PathBuf;

use crossterm::event::{self, Event};
use ratatui::DefaultTerminal;
use vaultrs::{app::App, data::load_projects, ui};

fn main() -> io::Result<()> {
    let root = vault_root()?;
    let projects = load_projects(&root)?;

    let dump = std::env::args().any(|arg| arg == "--dump");
    if dump {
        for project in projects {
            println!("{}", project.name);
        }
        return Ok(());
    }
    ratatui::run(|terminal| run(terminal, App::new(projects)))
}

fn run(terminal: &mut DefaultTerminal, mut app: App) -> io::Result<()> {
    while !app.should_exit() {
        terminal.draw(|frame| ui::draw(frame, &app))?;
        if let Event::Key(key_event) = event::read()? {
            app.handle_key_event(key_event);
        }
    }
    Ok(())
}

fn vault_root() -> io::Result<PathBuf> {
    let home_dir = match std::env::var("HOME") {
        Ok(home_dir) => home_dir,
        Err(_) => {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "home directory not set",
            ));
        }
    };
    let home_path = PathBuf::from(home_dir);
    let vault_dir = home_path.join("Documents/vault/projetos");
    Ok(vault_dir)
}
