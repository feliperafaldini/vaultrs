use crate::model::Project;
use std::fs;
use std::io;
use std::path::Path;

pub fn load_projects(root: &Path) -> io::Result<Vec<Project>> {
    let mut projects = Vec::new();
    let entries = fs::read_dir(root)?;
    for entry in entries {
        let dir_entry = match entry {
            Ok(dir_entry) => dir_entry,
            Err(_) => continue,
        };
        let file_type = match dir_entry.file_type() {
            Ok(file_type) => file_type,
            Err(_) => continue,
        };
        if !file_type.is_dir() {
            continue;
        }
        let name = match dir_entry.file_name().into_string() {
            Ok(name) => name,
            Err(_) => continue,
        };
        if name.starts_with('.') {
            continue;
        }
        projects.push(Project {
            name,
            path: dir_entry.path(),
        });
    }
    projects.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(projects)
}
