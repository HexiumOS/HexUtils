# HexUtils

**HexUtils** is a lightweight, cross-platform collection of command-line utilities designed to provide simple alternatives to common [coreutils] tools.

## Included Commands

- `cat`: Displays the contents of a file in the terminal.
- `echo`: Outputs the provided string to the terminal.
- `mkfile`: Creates an empty file with the specified name.
- `mkdir`: Crates an empty directory with the specified name.
- `rm`: Removes a file/directory with the specified name.

## Why HexUtils?
HexUtils offers minimal, fast, and portable replacements for some of the most frequently used Unix command-line tools, making it ideal for scripting, embedded environments, or learning purposes.

## Installation

Enter root directory and run

```
cargo build
```

The files will be in the target/debug folder.

[coreutils]: https://www.gnu.org/savannah-checkouts/gnu/coreutils/coreutils.html
