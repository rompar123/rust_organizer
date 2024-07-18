mod tree;

extern crate core;

use std::fs;
use std::fs::DirEntry;

fn main() -> std::io::Result<()>
{
    let mut folders = vec!();
    let mut files = vec!();
    for entry in fs::read_dir("/home/roman/projects/rust_organizer/example/")?
    {
        let entry= entry?;
        if entry.file_type()?.is_dir()
        {
            folders.push(entry);
        }
        else if entry.file_type()?.is_file()
        {
            files.push(entry)
        }
    } // TODO need to recursevely check all the folders.

    find_series(&files);

    Ok(())
}

fn find_series(file_list : &Vec<DirEntry>) -> tree::Tree
{
    let mut s = tree::Tree::new("/");
    for e in file_list
    {
        let name = e.file_name();
        s.insert(name.to_str().unwrap());
    }
    println!("Tree Test:\n{:?}", s);
    s
}
