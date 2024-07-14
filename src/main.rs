extern crate core;

use core::str;
use std::ffi::{OsString};
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

    print!("\n\n\n FINALLY: \n");
    println!("Folders: {:?}", folders);
    println!("files: {:?}", files);

    Ok(())
}

#[derive(Debug)]
struct Tree
{
    _inner : Vec<Tree>,
    _name : OsString,
    _count : i32,
}

impl Tree
{
    pub fn new(s : &str) -> Self
    {
        Self
        {
            _inner: vec!(),
            _name: OsString::from(s),
            _count: 0,
        }
    }

    fn add(&mut self, node : &str) -> ()
    {
        if node.is_empty()
        {
            return
        }
        println!("adding new node for '{}'", node);
        self._inner.push(Tree::new(node));
        self._count = self._count + 1;
    }

    fn find_common(node : &str, item : &str) -> usize
    {
        node.chars()
            .zip(item.chars())
            .position(|x| x.0 != x.1)
            .map(|index| index + 1)
            .unwrap_or(0)
    }

    /* should return the item with the most common amount of characters from the start
    */
    fn find_best(&mut self, item_name : &str) -> Option<&mut Tree>
    {
        self._inner.iter_mut().fold(
            (None, 0usize),
            |(first, best_match), current_node:&mut Tree| {
                let index = Self::find_common(current_node._name.to_str().unwrap(), item_name);
                if index > best_match
                {
                    return (Some(current_node), index);
                }
                return (first, best_match);
            }
        ).0
    }

    /// we should find the correct node to insert into recursively, if for some reason we break the current node in "half" the inner half should inherit the content and count...
    /// only when the "best" node is already found should we insert(serves as base case for recursion)
    /// ```
    ///
    /// ```
    pub fn insert(&mut self, item_name : &str) -> ()
    {
        let index = Self::find_common(self._name.to_str().unwrap(), item_name);
        if index == 0 // this is the root node and no commonality found, this is the recursion base base
        {
            self.add(item_name);
            self._count += 1;
            return
        }

        let suffix = &item_name[index-1..];
        let best_node = self.find_best(suffix);
        if best_node.is_some()
        {
            best_node.unwrap().insert(suffix);
        }
        else
        {
            self.add(suffix);
        }
        self._count += 1;

        let prefix = &item_name[..index-1];
        let node_index = Self::find_common(prefix, self._name.to_str().unwrap());
        let binding = self._name.clone();
        let node_suffix = &binding.to_str().unwrap()[node_index..];
        if !prefix.is_empty()
        {
            self._name = OsString::from(prefix);
        }
        self.add(node_suffix);
        self._count += 1;
    }
    /*{
        let _name = self._name.clone();
        let mut prefix = _name.as_os_str();
        let mut suffix= item_name.as_os_str();
        let mut tree_suffix= OsString::new();

        let mut best = (self as &Self, 0);
        for t in &self._inner
        {
            let index = Self::find_common(t._name.to_str().unwrap(), item_name.to_str().unwrap());
            if (index > best.1)
            {
                best = (t,index);
                continue;
            }
            prefix = OsStr::from_bytes(&item_name.as_bytes()[..index-1]);
            suffix = OsStr::from_bytes(&item_name.as_bytes()[index..]);
            tree_suffix = OsString::from(OsStr::from_bytes(&t._name.as_bytes()[index..]));
        }

        self.rebase(prefix);
        self.add(suffix.to_str().unwrap());
        self.add(tree_suffix.to_str().unwrap());
    }*/
}

fn find_series(file_list : &Vec<DirEntry>) -> Tree
{
    let mut s = Tree::new("/");
    for e in file_list
    {
        let name = e.file_name();
        s.insert(name.to_str().unwrap());
    }
    println!("Tree Test:\n{:?}", s);
    s
}