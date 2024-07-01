# Desk-exec

Execute programs defined in XDG desktop entries directly from the command line.
Allows for substring pattern based searching.

## Usage

### Execute a desktop entry

```sh
desk-exec <PROGRAM_NAME>
```

### Execute a desktop entry and detach it from the terminal

```sh
desk-exec --detach <PROGRAM_NAME>
```

### Execute a custom desktop entry

You may want to execute a program with extra arguments or behaviour. 
To do this, you can create a .desktop file at the default `$HOME/.local/share/applications`
or a custom directory set in the configuration file.

Entries found in directories specified in the configuration file will take precedence over
entries in the user applications folder, which takes precedence over the system applications folder.

```sh
# Executes the first match found
desk-exec --first-only <PROGRAM_NAME>
```

## Configuration

A default configuration file is generated at `$HOME/.config/desk-exec/desk_exec.toml`.

```toml
[search]
# Toggles the searching of the default XDG data directories
xdg_default_dirs = true

# List of custom search directories in order of precedence
dirs = []
```

## Installation

Currently, binaries for x86_64 are the only ones provided.

### ArchLinux

Desk-exec can be installed using your favorite AUR helper with any of the following packages.

* [desk-exec](https://aur.archlinux.org/packages/desk-exec)
* [desk-exec-bin](https://aur.archlinux.org/packages/desk-exec-bin)
* [desk-exec-git](https://aur.archlinux.org/packages/desk-exec-git)

```sh
paru -S (desk-exec, desk-exec-bin, desk-exec-git)
```

### Releases

Binary tar-balls can be found on the [releases page](https://github.com/AxerTheAxe/desk-exec/releases).

The `dist` folder will contain any extra stuff like shell completions and man page entries.

### Cargo

Desk-exec can also be built from source or installed with cargo from [crates.io](https://crates.io/crates/desk_exec).

```sh
cargo install desk-exec
```

## Contributing

If you have any suggestions or problems, please [submit an issue](https://github.com/AxerTheAxe/desk-exec/issues/new).
If you would like to contribute code, [pull requests are welcome](https://github.com/AxerTheAxe/desk-exec/compare).

## License

This project is licensed under the [Unlicense](LICENSE) license.
