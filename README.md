# rust-ls

A simple clone of the list function on linux/unix systems, created to familiarize myself with Rust

features include: different colors for directories, tilde expansion, and help menu

## Build
optional: clean the build space by running `cargo clean`
run `cargo build -r` in the source directory to build the release version, omit the -r if you want to build it in debug

## Usage: rust-ls [flags] [directory]

Supported flags:
- -a list all files, including hidden ones (files and directories that start with a period)
- -l list in long format, includes read status, uid, gid, size, modified date
- -F trailing slashes to directories
- -h display the help page
- -v display version

Example: list all files with trailing slashes in the home directory `rust-ls -aF ~`
