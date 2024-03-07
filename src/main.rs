use std::env;
use std::fs;

fn main() {
    let path = match env::current_dir() {
        Ok(path) => path,
        Err(e) => panic!("Could not get current working path: {e}"),
    };

    let dir_contents = match fs::read_dir(path) {
        Ok(contents) => contents,
        Err(e) => panic!("Could not read current directory: {e}"),
    };

    for item in dir_contents {
        match item {
            Ok(i) => println!("=> {:#?}", i),
            Err(e) => println!("Could not read item: {e}"),
        }
    }
}
