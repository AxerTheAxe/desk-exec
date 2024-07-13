use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Arguments {
    /// The name of or path to the desktop entry
    #[arg(group = "input")]
    pub program: Option<String>,

    /// Run the program in the background, detaching it from the terminal.
    #[arg(short, long, requires = "input")]
    pub detach: bool,

    /// Run the first matching entry without prompting
    #[arg(short, long, requires = "input")]
    pub first_only: bool,

    /// Path to a configuration file.
    #[arg(short, long, requires = "input")]
    pub config_file: Option<PathBuf>,

    /// Generate a default configuration file.
    #[arg(short, long)]
    pub generate_config: bool,

    /// Display all directories searched for desktop entries, in order of priority.
    #[arg(long)]
    pub list_dirs: bool,

    /// Display all available desktop entries
    #[arg(long)]
    pub list_entries: bool,
}
