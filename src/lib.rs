use ignore::DirEntry;

pub fn pretty_print_dir_entry(entry: DirEntry) {
    println!("{}", entry.path().display());
}
