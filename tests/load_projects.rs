use std::io::ErrorKind;
use std::path::Path;

use vaultrs::data::load_projects;

fn fixture() -> &'static Path {
    Path::new("tests/fixtures/projetos")
}

#[test]
fn loads_only_direct_directories_sorted() {
    let projects = load_projects(fixture()).expect("fixture should load");

    assert_eq!(projects.len(), 3, "project count");

    let mut names = Vec::new();
    for project in &projects {
        names.push(project.name.as_str());
    }
    assert_eq!(names, vec!["alpha", "beta", "zeta"], "names and order");

    assert_eq!(projects[0].path, fixture().join("alpha"));
    assert_eq!(projects[1].path, fixture().join("beta"));
    assert_eq!(projects[2].path, fixture().join("zeta"));
}

#[test]
fn empty_root_is_empty_list() {
    let projects =
        load_projects(Path::new("tests/fixtures/vazio")).expect("empty root should be Ok");
    assert!(projects.is_empty(), "empty root has no projects");
}

#[test]
fn missing_root_is_not_found() {
    let error = load_projects(Path::new("tests/fixtures/nao-existe"))
        .expect_err("missing root should fail");
    assert_eq!(error.kind(), ErrorKind::NotFound);
}

#[test]
fn file_as_root_is_error() {
    let result = load_projects(Path::new("tests/fixtures/arquivo.txt"));
    assert!(result.is_err(), "a file as root should fail");
}
