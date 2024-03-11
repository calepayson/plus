use std::fs::{self, File};
use std::io::{self, Read};
use std::path::PathBuf;

pub fn print_directory_contents(
    path: PathBuf, 
    depth: u32, 
    patterns: &Vec<String>
) -> io::Result<()> {

    print_current_root(&path, &depth)?;

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            print_directory_contents(path, depth + 1, patterns)?;
        } else {
            let depth = depth + 1;
            print_current_root(&path, &depth)?;
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

fn get_file_name_from_path(path: &PathBuf) -> String {
    path.file_name().unwrap().to_string_lossy().to_string()
}

// fn check_is_not_dot_file(path: &PathBuf) -> bool{
// }

pub fn list_git_ignore() -> Result<Vec<String>, io::Error> {
    let gitignore_string = read_git_ignore()?;
    let gitignore_iterator = gitignore_string.split("\n");

    let mut files_to_ignore: Vec<String> = Vec::new();
    
    for line in gitignore_iterator {
        let line = line.to_string();
        files_to_ignore.push(line);
    }

    Ok(files_to_ignore)
}

fn read_git_ignore() -> Result<String, io::Error> {
    let mut f = File::open(".gitignore")?;
    let mut data = String::new();

    f.read_to_string(&mut data)?;

    Ok(data)
}

// fn check_if_should_ignore(path: &PathBuf, patterns: &Vec<String>) -> bool {
// }

