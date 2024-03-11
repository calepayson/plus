use ignore::Walk;
use std::env;

use plus::run;

fn main() {
    let path = match env::current_dir() {
        Ok(path) => path,
        Err(e) => panic!("Could not get current working path: {e}"),
    };

    for result in Walk::new(path) {
        match result {
            Ok(entry) => run::pretty_print_dir_entry(entry),
            Err(err) => println!("ERROR: {}", err),
        }
    }
}

