use std::{
    env,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

use desk_exec::CleanPlaceholders;

#[test]
fn default_dirs() {
    let dirs = desk_exec::get_default_entry_dirs()
        .expect("could not get XDG base application directories");
    let dirs_comp = freedesktop_desktop_entry::default_paths();

    assert_eq!(dirs, dirs_comp);
}

#[test]
fn execute_entry() {
    let entry_dir = create_entry("ls.desktop", "ls").expect("could not create entry file");

    let entry = &desk_exec::search_for_entries("ls", &[entry_dir], &["en_us".to_string()], true)
        .expect("could not search for entries")[0];

    desk_exec::exec_entry(&entry, true).expect("could not execute entry");
}

#[test]
fn clean_placeholders() {
    let entry_dir = create_entry("clean_placeholders.desktop", "dum%fmy%U%F")
        .expect("could not create entry file");

    let entry = &desk_exec::search_for_entries(
        "clean_placeholders",
        &[entry_dir],
        &["en_us".to_string()],
        true,
    )
    .expect("could not search for entries")[0];

    assert_eq!(entry.exec_clean().unwrap(), "dummy");
}

fn create_entry(name: &str, exec: &str) -> Option<PathBuf> {
    let out_env = env::var("OUT_DIR").ok()?;
    let out_dir = Path::new(&out_env);
    let entry_path = out_dir.join(name);

    let entry_data = format!(
        r#"
        [Desktop Entry]
        Name={name}
        Exec={exec}
    "#
    );

    let mut entry = File::create(&entry_path).ok()?;
    entry.write_all(entry_data.as_bytes()).ok()?;

    Some(out_dir.to_path_buf())
}
