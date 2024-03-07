use std::env;

fn main() {
    let path = match env::current_dir() {
        Ok(path) => path,
        Err(e) => panic!("Could not get current working path: {e}"),
    };

    println!("The current directory is: {:?}", path);
}
