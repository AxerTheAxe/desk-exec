# Desk-exec

Execute programs defined in XDG desktop entries directly from the command line.
Allows for substring pattern based searching.

## Usage

### Execute a desktop entry

`desk-exec <PROGRAM_NAME>`

### Execute a desktop entry and detach it from the terminal

`desk-exec --detach <PROGRAM_NAME>`

## Installation

Currently, binaries for x86_64 are the only ones provided.

### ArchLinux

Desk-exec can be installed using your favorite AUR helper with any of the following packages.

* [desk-exec](https://aur.archlinux.org/packages/desk-exec)
* [desk-exec-bin](https://aur.archlinux.org/packages/desk-exec-bin)
* [desk-exec-git](https://aur.archlinux.org/packages/desk-exec-git)

`paru -S (desk-exec, desk-exec-bin, desk-exec-git)`

### Releases

Binary tar-balls can be found on the [releases page.](https://github.com/AxerTheAxe/desk-exec/releases)

The `dist` folder will contain any extra stuff like shell completions and man page entries.

### Cargo

Desk-exec can also be built from source or installed with cargo from [crates.io](https://crates.io/crates/desk_exec)

`cargo install desk-exec`

## Contributing

If you have any suggestions or problems, please [submit an issue](https://github.com/AxerTheAxe/desk-exec/issues/new).
If you would like to contribute code, [pull requests are welcome](https://github.com/AxerTheAxe/desk-exec/compare).

## License

This project is licensed under the [Unlicense](LICENSE) license.
