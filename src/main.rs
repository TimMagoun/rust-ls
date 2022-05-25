use clap::{Arg, Command};
use std::fs::{self, DirEntry};
use std::path::Path;

fn main() {
    // Set up the command line parser
    let matches = Command::new("Rust ls")
        .version("0.0.1")
        .author("Tim Magoun")
        .about("Lists files in a directory")
        .arg(
            Arg::new("all")
                .short('a')
                .takes_value(false)
                .help("List all files in the current directory, including hidden files"),
        )
        .arg(
            Arg::new("Trailing")
                .short('F')
                .takes_value(false)
                .help("List files with trailing / added to directory names"),
        )
        .arg(
            Arg::new("long")
                .short('l')
                .takes_value(false)
                .help("Lists in the long format"),
        )
        .arg(
            Arg::new("sort")
                .short('S')
                .takes_value(false)
                .help("Sorts the list by size (descending)"),
        )
        .arg(
            Arg::new("dir")
                .value_name("DIR")
                .default_value(".")
                .help("The directory to list"),
        )
        .get_matches();

    // Set up the directory path expansion
    let dir_name = matches.value_of("dir").unwrap_or("");
    // println!("Dir name: {:?}", dir_name);
    let expanded_dir_name = shellexpand::tilde(dir_name).to_string();
    let expanded_dir_path = Path::new(&expanded_dir_name);

    // Vector of DirEntry type that stores all files in the directory
    let mut file_vec = Vec::<DirEntry>::new();
    if expanded_dir_path.is_dir() {
        for entry in fs::read_dir(expanded_dir_path).unwrap() {
            let entry = entry.unwrap();
            file_vec.push(entry);
        }
    } else {
        println!("Argument given is not a directory!");
        return;
    }

    // Iterate through the files to print out their names
    for f in &file_vec {
        let entry_path = f.path();
        let file_name_str = entry_path.file_name().unwrap_or_default().to_string_lossy();
        // Only display non-hidden files unless the -a flag is on
        if matches.is_present("all") || !file_name_str.starts_with('.') {
            if entry_path.is_dir() {
                println! {"\x1b[93m{}\x1b[0m", file_name_str};
            } else {
                println!("{}", file_name_str);
            }
        }
    }
}
