use std::collections::HashSet;
use std::ffi::OsString;
use std::fs;
use std::fs::DirEntry;

fn main() -> std::io::Result<()>
{
    let mut folders = vec!();
    let mut files = vec!();
    for entry in fs::read_dir("/hdd1/Media/hdd_trans_downloads/")?
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
    }

    find_series(&files);

    print!("\n\n\n FINALLY: \n");
    println!("Folders: {:?}", folders);
    println!("files: {:?}", files);

    Ok(())
}

fn find_series(file_list : &Vec<DirEntry>) -> Box<HashSet<OsString>>
{
    let mut s = HashSet::new();
    for e in file_list
    {
        let name = e.file_name();
    }
    return Box::new(s);
}