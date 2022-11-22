use std::fmt::{Debug, Formatter, Result};

pub const DJB2_HASH: usize = 5381; 
pub const HASH_SIZE: usize = 100;

pub fn dbj2(key: &str) -> u64 {
    let bytes = key.as_bytes();
    let mut hash = DJB2_HASH as u64;

    for byte in bytes {
        hash = (hash << 5).wrapping_add(hash).wrapping_add(*byte as u64);
    }

    return hash
}

pub struct HashNode<'a, T> {
    key: &'a str,
    value: T,
    next: Option<Box<HashNode<'a, T>>>
}

impl<'a, T> HashNode<'a, T> {
    pub fn create_node(key: &'a str, value: T) -> Option<Box<HashNode<'a, T>>> {
        Some(Box::new(HashNode { key, value, next: None }))
    }

    pub fn insert_next_node(node: &mut Box<HashNode<'a, T>>, next: Option<Box<HashNode<'a, T>>>) {
        match &mut node.next {
            Some(x) => HashNode::insert_next_node(x, next),
            None => node.next = next
        }
    }

    pub fn search_node(&mut self, key: &'a str) -> Option<&T> {
        if self.key == key {
            return Some(&self.value);
        }

        match &mut self.next {
            Some(x) => x.search_node(key),
            None => None,
        }
    }
}

pub struct HashTable<'a, T> {
    table: [Option<Box<HashNode<'a, T>>>; HASH_SIZE],
}

impl<'a, T> HashTable<'a, T> {
    // Rust really needs an builtin function to do this [Option; SIZE] stuff
    pub fn new() -> Self {
        Self { table: [
            None, None, None, None, None, None, None,
            None, None, None, None, None, None, None,
            None, None, None, None, None, None, None,
            None, None, None, None, None, None, None,
            None, None, None, None, None, None, None,
            None, None, None, None, None, None, None,
            None, None, None, None, None, None, None,
            None, None, None, None, None, None, None,
            None, None, None, None, None, None, None,
            None, None, None, None, None, None, None,
            None, None, None, None, None, None, None,
            None, None, None, None, None, None, None,
            None, None, None, None, None, None, None,
            None, None, None, None, None, None, None,
            None, None
          ] }
    }

    pub fn insert(&mut self, key: &'a str, value: T) {
        let hash_key = dbj2(key) as usize % HASH_SIZE;

        match &mut self.table[hash_key] {
            Some(x) => HashNode::insert_next_node(x, HashNode::create_node(key, value)),
            None => self.table[hash_key] = HashNode::create_node(key, value),
        }
    }

    pub fn search(&mut self, key: &'a str) -> Option<&T> {
        let hash_key = dbj2(key) as usize % HASH_SIZE;

        match &mut self.table[hash_key] {
            Some(x) => x.search_node(key),
            None => None
        }
    }
}

pub struct User<'a> {
    name: &'a str,
    age: usize
}

impl Debug for User<'static> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("User")
         .field("name", &self.name)
         .field("age", &self.age)
         .finish()
    }
}

fn main() {
    let mut users: HashTable<User> = HashTable::new();

    users.insert("mingas", User { name: "mingas", age: 19 });
    users.insert("guizin", User { name: "guizin", age: 19 });
    users.insert("vinizin", User { name: "vinizin", age: 190 });
     
    println!("{:?}", users.search("mingas"));
    println!("{:?}", users.search("guizin"));
    println!("{:?}", users.search("vinizin"));
}
