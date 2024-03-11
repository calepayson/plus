use ignore::DirEntry;
use std::format;

pub fn pretty_print_dir_entry(entry: DirEntry) {
    println!(
        "{}{}", 
        get_formatting_chars(&entry), 
        get_file_name(&entry)
    );
}

fn get_formatting_chars(entry: &DirEntry) -> String {
    let depth = entry.depth();
    let spaces = " ".repeat(depth * 4);
    let delimeter = "-";

    format!("{}{}", spaces, delimeter)
}

fn get_file_name(entry: &DirEntry) -> String {
    let file_name = entry.file_name().to_str().unwrap().to_string();

    file_name
}

#[cfg(test)]
mod tests {

}
