use std::env;
use std::path::PathBuf;

fn main() {
    let path_buff: PathBuf = env::current_dir().unwrap();

    let path = path_buff.to_str().expect("error"); 

    println!("The current directory is: {path}");
}
