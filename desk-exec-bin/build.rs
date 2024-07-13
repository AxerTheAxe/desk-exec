use std::{
    env, fs,
    io::{ErrorKind, Result},
    path::{Path, PathBuf},
};

use args::Arguments;
use clap::{Command, CommandFactory, ValueEnum};
use clap_complete::Shell;
use clap_complete_nushell::Nushell;
use clap_mangen::Man;

#[path = "src/args.rs"]
mod args;

fn main() -> Result<()> {
    let out_dir =
        PathBuf::from(env::var_os("OUT_DIR").ok_or(ErrorKind::NotFound)?).join("../../../dist");
    if !out_dir.exists() {
        fs::create_dir(&out_dir).unwrap();
    }

    let mut cmd = Arguments::command();

    gen_manpage(&cmd, &out_dir)?;
    gen_completions(&mut cmd, &out_dir)?;

    Ok(())
}

fn gen_manpage(cmd: &Command, out_dir: &Path) -> Result<()> {
    let man = Man::new(cmd.clone());

    let mut buffer = Vec::new();
    man.render(&mut buffer)?;

    fs::write(out_dir.join("desk-exec.1"), buffer)?;

    Ok(())
}

fn gen_completions(cmd: &mut Command, out_dir: &Path) -> Result<()> {
    for &shell in Shell::value_variants() {
        if let Shell::PowerShell = shell {
            continue;
        }

        clap_complete::generate_to(shell, cmd, "desk-exec", out_dir)?;
    }
    clap_complete::generate_to(Nushell, cmd, "desk-exec", out_dir)?;

    Ok(())
}
