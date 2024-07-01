use std::{env, fs::File, io::Write, path::Path};

#[test]
fn default_dirs() {
    let dirs = desk_exec::get_default_entry_dirs()
        .expect("could not get XDG base application directories");
    let dirs_comp = freedesktop_desktop_entry::default_paths();

    assert_eq!(dirs, dirs_comp);
}

#[test]
fn create_and_execute_entry() {
    let out_dir = env::var("OUT_DIR").expect("could not get Cargo 'OUT_DIR' environment variable");
    let temp_dir = Path::new(&out_dir);

    let entry_data = r#"
        [Desktop Entry]
        Name=ls
        Exec=ls
    "#;

    let mut file = File::create_new(temp_dir.join("ls.desktop"))
        .expect("failed to create a file in the temporary directory");
    file.write_all(entry_data.as_bytes())
        .expect("could not write at the file in the temporary directory");

    let entry = &desk_exec::search_for_entries(
        "ls",
        &[temp_dir.to_path_buf()],
        &["en_us".to_string()],
        true,
    )
    .expect("could not find the temporary entry.")[0];

    desk_exec::exec_entry(&entry, true).expect("could not execute the temporary deskopt entry");
}
