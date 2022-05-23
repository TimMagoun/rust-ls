use std::env;
use std::fs;

fn main() {
    println!("Rust ls");
    // Read the flags and the target directory
    let args: Vec<String> = env::args().collect();
    let arg_len = args.len();

    // Find the directory to perform the list on
    let dir_str = &args[arg_len-1];

    // Find collection of files in the directory
    // Make sure the input is valid
    let dir_md = fs::metadata(dir_str);
    if dir_md.is_err() || !dir_md.unwrap().is_dir(){
        println!("Input {} is not a directory", &dir_str);
        return;
    }

    let paths = fs::read_dir(dir_str).unwrap();

    for path in paths{
        println!("Name: {}", path.unwrap().path().display());
    }
  print!("End");  

}
