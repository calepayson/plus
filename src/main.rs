use std::env;

fn main() {
    let path = env::current_dir();
    println!("The current directory is {}", path.expect("Some error").display());
}
