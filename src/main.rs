use ignore::Walk;
use std::env;
use std::io;

use plus::pretty_print_dir_entry;

fn main() -> io::Result<()> {
    let path = match env::current_dir() {
        Ok(path) => path,
        Err(e) => panic!("Could not get current working path: {e}"),
    };

    for result in Walk::new(path) {
        match result {
            Ok(entry) => pretty_print_dir_entry(entry),
            Err(err) => println!("ERROR: {}", err),
        }
    }

    Ok(())
}

