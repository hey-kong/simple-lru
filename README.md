# simple-lru
The simple implementation of least-recently-used (LRU) cache to learn Rust.

## Usage
Add to dependencies:
```
[dependencies]
simple-lru = { git = "https://github.com/hey-kong/simple-lru", branch = "main" }
```
Example:
```rust
extern crate simple_lru;

use simple_lru::LRUCache;

fn main() {
    let on_evicted = |key, value| {
        println!(
            "delete key-value pair: {{ key: \"{}\", val: \"{}\" }}",
            key, value
        );
    };

    let mut cache = LRUCache::new(10, Some(on_evicted));
    cache.set(String::from("0"), String::from("0"));
    cache.set(String::from("1"), String::from("1"));
    cache.set(String::from("2"), String::from("2"));
    cache.set(String::from("3"), String::from("3"));
    assert_eq!(cache.get(String::from("0")), Some(String::from("0")));
    assert_eq!(cache.get(String::from("1")), Some(String::from("1")));
    cache.set(String::from("4"), String::from("4"));
    cache.set(String::from("5"), String::from("5"));
    assert_eq!(cache.get(String::from("0")), Some(String::from("0")));
    assert_eq!(cache.get(String::from("1")), Some(String::from("1")));
    assert_eq!(cache.get(String::from("2")), None);
    assert_eq!(cache.get(String::from("3")), Some(String::from("3")));
    assert_eq!(cache.get(String::from("4")), Some(String::from("4")));
    assert_eq!(cache.get(String::from("5")), Some(String::from("5")));
}
```