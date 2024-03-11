use std::env;
use std::io;

use plus::print_directory_contents;
use plus::list_git_ignore;

fn main() -> io::Result<()> {
    let path = match env::current_dir() {
        Ok(path) => path,
        Err(e) => panic!("Could not get current working path: {e}"),
    };

    let files_to_ignore = list_git_ignore()?;
    
    print_directory_contents(path, 0, &files_to_ignore)?;

    Ok(())
}
