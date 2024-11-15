//! # Desk-exec
//!
//! Desk-exec provides an API to search for and execute programs from their XDG desktop entry
//! files.
//!
//! ## Overview
//!
//! This API is designed around the 'DesktopEntry' struct from the 'freedesktop_desktop_entry'
//! crate. It provides the functionality to do a substring search for list of directories, and
//! execute the program within a desktop entry.

use std::{
    borrow::Cow,
    ops::Not,
    panic,
    path::{Path, PathBuf},
    process::{Command, ExitStatus, Stdio},
    result,
};

use freedesktop_desktop_entry::{DesktopEntry, Iter};
use regex::Regex;
use thiserror::Error;

pub type Result<T, E = DesktopEntryError> = result::Result<T, E>;

/// Represents all possible error variants when trying to execute a desktop entry
#[derive(Debug, Error)]
pub enum DesktopEntryError {
    #[error("failed to find the 'Exec' value at '{0}'")]
    MissingExec(PathBuf),

    #[error("failed to execute the program '{0}' at '{1}'")]
    InvalidExec(String, PathBuf),

    #[error("failed to parse the 'Exec' value at '{0}'")]
    InvalidExecSyntax(PathBuf),

    #[error("internal regex error when parsing the 'Exec' value at {0}")]
    Regex(PathBuf),
}

/// Gives the ability for a desktop entry type to provide an executable name cleaned from any
/// placeholder values
pub trait CleanPlaceholders {
    fn exec_clean(&self) -> Result<Cow<str>>;
}

impl<'a> CleanPlaceholders for DesktopEntry<'a> {
    /// Returns the name of the entries executable without any placeholder values.
    fn exec_clean(&self) -> Result<Cow<str>> {
        let output = Regex::new("%[a-zA-Z]")
            .map_err(|_| DesktopEntryError::Regex(self.path.to_path_buf()))?;

        Ok(output.replace_all(
            self.exec()
                .ok_or_else(|| DesktopEntryError::MissingExec(self.path.to_path_buf()))?,
            "",
        ))
    }
}

/// Panic-less wrapper for 'fredesktop_desktop_entry::default_paths()'. Returns the default XDG
/// data directories, where desktop entries are stored on most systems.
pub fn get_default_entry_dirs() -> Option<impl Iterator<Item = PathBuf>> {
    panic::catch_unwind(freedesktop_desktop_entry::default_paths).ok()
}

/// Searches a list of directories for any desktop entry files that match the 'program_name'
/// pattern. Will return early with the first match found when 'get_first' is true.
///
/// # Examples
///
/// ```
/// use desk_exec::{search_for_entries, get_default_entry_dirs};
/// use freedesktop_desktop_entry::get_languages_from_env;
///
/// let dirs = get_default_entry_dirs().unwrap();
/// let locales = get_languages_from_env();
///
/// let entries = search_for_entries("program", dirs, &locales, false).unwrap_or_default();
///
/// for entry in entries {
///     println!("{}", entry.path.display());
/// }
/// ```
pub fn search_for_entries<'a>(
    program_name: &str,
    dirs: impl Iterator<Item = PathBuf>,
    locales: &[String],
    get_first: bool,
) -> Option<Vec<DesktopEntry<'a>>> {
    let mut entries = Vec::new();
    for file in Iter::new(dirs) {
        let entry = match DesktopEntry::from_path(file.clone(), Some(locales)) {
            Ok(entry) => entry,
            Err(_) => continue,
        };

        if let (Ok(program_path), Ok(file_path)) =
            (Path::new(program_name).canonicalize(), file.canonicalize())
        {
            if program_path == file_path {
                entries.clear();
                entries.push(entry);
                return Some(entries);
            }
        }

        if entry.no_display() {
            continue;
        }

        if match_entry_name(program_name, &entry, locales) {
            entries.push(entry);
            if get_first {
                return Some(entries);
            }
        }
    }

    entries.is_empty().not().then_some(entries)
}

/// Attempts to execute the program within a desktop entry. The executed program will be detached
/// from the terminal if 'detach' is true.
///
/// # Returns
///
/// If execution is successful and the program is not detached, the 'ExitStatus' of the executed
/// program will be returned.
///
/// # Examples
///
/// ```
/// use desk_exec::exec_entry;
/// use freedesktop_desktop_entry::DesktopEntry;
///
/// let entry = DesktopEntry::from_appid("example_appid");
///
/// match exec_entry(&entry, false) {
///     Ok(Some(exit_status)) => {
///         eprintln!("Program executed with code: '{}'", exit_status.code().unwrap_or_default());
///     }
///     Ok(None) => {
///         eprintln!("Program executed with no exit code.");
///     }
///     Err(_) => {
///         eprintln!("Program failed to execute.");
///     }
/// }
/// ```
pub fn exec_entry(entry: &DesktopEntry, detach: bool) -> Result<Option<ExitStatus>> {
    let entry_exec_cmd = entry.exec_clean()?.to_string();
    let mut entry_exec = entry_exec_cmd.split_whitespace();

    let cmd = entry_exec
        .next()
        .ok_or_else(|| DesktopEntryError::InvalidExecSyntax(entry.path.to_path_buf()))?;
    let args: Vec<&str> = entry_exec.collect();

    let mut exec = Command::new(cmd);
    exec.args(&args);

    if detach {
        exec.stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|_| {
                DesktopEntryError::InvalidExec(entry_exec_cmd, entry.path.to_path_buf())
            })?;
        Ok(None)
    } else {
        let status = exec.status().map_err(|_| {
            DesktopEntryError::InvalidExec(entry_exec_cmd, entry.path.to_path_buf())
        })?;
        Ok(Some(status))
    }
}

fn match_entry_name(program_name: &str, entry: &DesktopEntry, locales: &[String]) -> bool {
    let program_name = program_name.to_lowercase();

    entry
        .name(locales)
        .map_or(false, |name| name.to_lowercase().contains(&program_name))
        || entry.appid.to_lowercase().contains(&program_name)
        || entry
            .generic_name(locales)
            .map_or(false, |name| name.to_lowercase().contains(&program_name))
}
