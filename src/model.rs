use std::path::PathBuf;

#[derive(Debug)]
pub struct Project {
    pub name: String,
    pub path: PathBuf,
}
