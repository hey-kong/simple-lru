use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;

use super::*;
use linked_list::{LinkedList, LinkedListNode};

#[derive(Debug)]
pub struct Entry {
    key: String,
    val: String,
}

impl Entry {
    pub fn new(key: String, value: String) -> Self {
        Self {
            key: key,
            val: value,
        }
    }

    pub fn size(&self) -> u64 {
        (self.key.len() + self.val.len()) as u64
    }
}

#[derive(Debug)]
pub struct LRUCache {
    max_bytes: u64,
    cur_bytes: u64,
    list: LinkedList<Entry>,
    map: HashMap<String, Rc<RefCell<LinkedListNode<Entry>>>>,
    on_evicted: Option<fn(String, String)>,
}

impl LRUCache {
    pub fn new(max_bytes: u64, on_evicted: Option<fn(String, String)>) -> Self {
        Self {
            max_bytes: max_bytes,
            cur_bytes: 0,
            list: LinkedList::new(),
            map: HashMap::new(),
            on_evicted: on_evicted,
        }
    }

    pub fn get(&mut self, key: String) -> Option<String> {
        if let Some(entry) = self.map.get(&key) {
            self.list.move_to_back(entry);
            return Some(entry.borrow().value.val.clone());
        }
        None
    }

    pub fn set(&mut self, key: String, value: String) {
        if let Some(entry) = self.map.get(&key) {
            self.list.move_to_back(entry);
            self.cur_bytes -= entry.borrow().value.size();
            entry.borrow_mut().value.val = value;
            self.cur_bytes += entry.borrow().value.size();
        } else {
            let entry = self.list.push(Entry::new(key.clone(), value));
            self.cur_bytes += entry.borrow().value.size();
            self.map.insert(key, entry);
        }

        while self.max_bytes != 0 && self.max_bytes < self.cur_bytes {
            self.remove()
        }
        println!("cur_bytes: {}", self.cur_bytes);
    }

    pub fn remove(&mut self) {
        if let Some(entry) = self.list.remove_front() {
            self.cur_bytes -= entry.borrow().value.size();
            self.map.remove(&entry.borrow().value.key);
            if let Some(on_evicted) = self.on_evicted.as_ref() {
                on_evicted(
                    entry.borrow().value.key.clone(),
                    entry.borrow().value.val.clone(),
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get() {
        let mut cache = LRUCache::new(1024, None);
        cache.set(String::from("key1"), String::from("123"));
        assert_eq!(cache.get(String::from("key1")), Some(String::from("123")));
        assert_eq!(cache.get(String::from("key2")), None);
    }

    #[test]
    fn test_set() {
        let mut cache = LRUCache::new(1024, None);
        cache.set(String::from("key"), String::from("1"));
        cache.set(String::from("key"), String::from("111"));
        assert_eq!(
            cache.cur_bytes,
            (String::from("key").len() + String::from("111").len()) as u64
        );
    }

    #[test]
    fn test_remove_oldest() {
        let mut cache = LRUCache::new(6, None);
        cache.set(String::from("0"), String::from("0"));
        cache.set(String::from("1"), String::from("1"));
        cache.set(String::from("2"), String::from("2"));
        cache.set(String::from("3"), String::from("3"));
        assert_eq!(cache.get(String::from("0")), None);
        assert_eq!(cache.get(String::from("1")), Some(String::from("1")));
        assert_eq!(cache.get(String::from("2")), Some(String::from("2")));
        assert_eq!(cache.get(String::from("3")), Some(String::from("3")));
    }
}
