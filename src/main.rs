mod args;
mod config;

use std::{io, path::PathBuf};

use anyhow::{Context, Result};
use args::Arguments;
use clap::Parser;
use config::Config;
use freedesktop_desktop_entry::DesktopEntry;

fn main() -> Result<()> {
    let arguments = Arguments::parse();

    if arguments.generate_config {
        config::create_default_config_file().context("failed to create config file")?;
        return Ok(());
    }

    let mut config = Config::try_new(arguments.config_file.as_deref())
        .context("failed to create or read a config file")?;

    config.populate_dirs();
    config.clean_dirs();

    let dirs = config.get_dirs().context("no existing directories found")?;

    if arguments.list_dirs {
        print_search_dirs(dirs);
        return Ok(());
    }

    let locales = freedesktop_desktop_entry::get_languages_from_env();
    let entries = desk_exec::search_for_entries(
        &arguments.program.unwrap_or_default(),
        dirs,
        &locales,
        arguments.first_only,
    )
    .context("no entries found")?;
    let entry = select_entry(&entries).context("failed to select an entry")?;

    if arguments.list_entries {
        print_entries(&entries);
        return Ok(());
    }

    let program_status = desk_exec::exec_entry(entry, arguments.detach)
        .context("failed to execute the specified program")?;

    std::process::exit(
        program_status
            .unwrap_or_default()
            .code()
            .unwrap_or_default(),
    );
}

fn select_entry<'a>(entries: &'a [DesktopEntry]) -> Result<&'a DesktopEntry<'a>> {
    if entries.len() == 1 {
        return Ok(entries
            .first()
            .context("could not get the first entry from the list")?);
    }

    let index_width = entries.len().to_string().len();
    loop {
        println!("Multiple entries found. Please enter a number.");
        for (index, entry) in entries.into_iter().enumerate() {
            println!("   {:index_width$}: {}", index, entry.path.display(),);
        }

        let mut input_buffer = String::new();
        io::stdin()
            .read_line(&mut input_buffer)
            .context("failed to read user input")?;

        match input_buffer.trim().parse::<usize>() {
            Ok(entry_index) if entry_index < entries.len() => {
                return Ok(entries.get(entry_index).context(format!(
                    "could not get an entry from index '{}'",
                    entry_index
                ))?);
            }

            Ok(_) => eprintln!("\nInvalid number. Please try again.\n"),
            Err(_) => eprintln!("\nInvalid input. Please try again.\n"),
        }
    }
}

fn print_search_dirs(dirs: &[PathBuf]) {
    let mut dirs = dirs.to_vec();
    dirs.reverse();

    if dirs.is_empty() {
        return;
    }

    for dir in dirs {
        let directory_path = dir.to_str();
        if let Some(path) = directory_path {
            println!("{path}");
        }
    }
}

fn print_entries(entries: &[DesktopEntry]) {
    for entry in entries {
        println!("{}", entry.path.display());
    }
}
