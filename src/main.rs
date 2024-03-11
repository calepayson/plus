use ignore::Walk;
use std::env;
use std::io;

fn main() -> io::Result<()> {
    let path = match env::current_dir() {
        Ok(path) => path,
        Err(e) => panic!("Could not get current working path: {e}"),
    };

    for result in Walk::new(path) {
        match result {
            Ok(entry) => println!("{}", entry.file_name().to_str().unwrap()),
            Err(err) => println!("ERROR: {}", err),
        }
    }

    Ok(())
}
