use std::env;
use std::path::PathBuf;

fn main() {
    let path = get_current_path();

    println!("The current directory is: {path}");
}

fn get_current_path() -> String {
    let path_buff: PathBuf = env::current_dir().unwrap();

    path_buff.to_str().unwrap().to_string()
}
