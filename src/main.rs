use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;

fn main() -> io::Result<()> {
    let path = match env::current_dir() {
        Ok(path) => path,
        Err(e) => panic!("Could not get current working path: {e}"),
    };
    
    print_directory_contents(path, 0)?;
    Ok(())
}

fn print_directory_contents(path: PathBuf, depth: u32) -> io::Result<()> {
    print_current_root(&path, &depth)?;

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() & check_is_not_dot_file(&path){
            print_directory_contents(path, depth + 1)?;
        }
    }
    Ok(())
}

fn print_current_root(path: &PathBuf, depth: &u32) -> io::Result<()> {
    let file_name = get_file_name_from_path(path);
    println!(
        "{}∟{}",
        " ".repeat(*depth as usize * 4),
        file_name
    );

    Ok(())
}

fn check_is_not_dot_file(path: &PathBuf) -> bool{
    let file_name = get_file_name_from_path(path);
    let first_char = file_name.chars().next();
    if first_char == Some('.') {
        return false
    }
    true
}

fn get_file_name_from_path(path: &PathBuf) -> String {
    path.file_name().unwrap().to_string_lossy().to_string()
}

// fn list_git_ignore() -> Vec<String> {
// 
// }
// 
// fn check_if_should_ignore(file: &str, &Vec<String>) -> bool {
// 
// }
