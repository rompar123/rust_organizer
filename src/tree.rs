use std::ffi::OsString;

#[derive(Debug)]
pub struct Tree
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
        println!("\t\tadding new node for '{}'", node);
        self._inner.push(Tree::new(node));
        self._count = self._count + 1;
    }

    fn find_common(_node : &str, _item : &str) -> usize
    {
        let min_index = std::cmp::min(_node.len(), _item.len());
        let node = &_node[..min_index];
        let item = &_item[..min_index];
        let mut some_zip = node.chars()
            .zip(item.chars());
        let some_index = some_zip.position(|x| x.0 != x.1);

        println!("--- returing index: {:?}", some_index);
        some_index.unwrap_or(min_index)
    }

    /* should return the item with the most common amount of characters from the start
    */
    fn find_best(&mut self, item_name : &str) -> Option<&mut Tree>
    {
        self._inner.iter_mut().fold(
            (None, Self::find_common(self._name.to_str().unwrap(), item_name)),
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

    pub fn insert(&mut self, item_name : &str) -> ()
    {
        println!("function call insert {} for {:?}", item_name, self._name);
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
            let best_node = best_node.unwrap();
            println!("\tcalling insert {} for {:?}", &suffix, best_node._name);
            best_node.insert(suffix);
            return
        }
        else
        {
            println!("\tcalling add {} for {:?}", suffix, self._name);
            self.add(suffix);
            self._count += 1;
        }

        let prefix = &item_name[..index-1];
        let node_index = Self::find_common(self._name.to_str().unwrap(), prefix);
        let binding = self._name.clone();
        let node_suffix = &binding.to_str().unwrap()[node_index..];
        if !prefix.is_empty()
        {
            println!("\trenaming {:?} t o{:?}", self._name, prefix);
            self._name = OsString::from(prefix);
        }
        println!("\tcalling add#2 {} for {:?}", suffix, self._name);
        self.add(node_suffix);
        self._count += 1;
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_tree_find_common()
    {
        assert_eq!(&"test"[..2], "te");
        assert_eq!(Tree::find_common("te", "test"), 2usize);
        assert_eq!(Tree::find_common("a", "b"), 0usize);
    }
}