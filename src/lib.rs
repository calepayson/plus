use ignore::DirEntry;
use std::format;

pub fn pretty_print_dir_entry(entry: DirEntry) {
    println!("{}{}", get_formatting_chars(&entry), entry.path().display());
}

pub fn get_formatting_chars(entry: &DirEntry) -> String {
    let depth = entry.depth();
    let spaces = " ".repeat(depth * 4);
    let delimeter = "-";

    format!("{}{}", spaces, delimeter)
}

#[cfg(test)]
mod tests {
    #[test]
    fn tests_work() {
        assert_eq!(1 + 1, 2);
    }
}
