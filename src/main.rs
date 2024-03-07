use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let path = match env::current_dir() {
        Ok(path) => path,
        Err(e) => panic!("Could not get current working path: {e}"),
    };
    
    print_directory_contents(path, 0);
}

fn print_directory_contents(path: PathBuf, depth: u32) {
    println!("{:?}", path.file_name().unwrap());

    let directory_contents = match fs::read_dir(path) {
        Ok(contents) => contents,
        Err(e) => panic!("Could not read current directory: {e}"),
    };

    for item in directory_contents {
        match item {
            Ok(i) => println!("{:?} at depth: {}", i.path(), depth),
            Err(e) => println!("Could not read item: {e}"),
        }
    }
}

