mod hash_map_type;
use hash_map_type::HashTableTrait;
use hash_map_type::HashTable;
use hash_map_type::hash;
use hash_map_type::Entry;
use std::mem;

impl HashTableTrait<u32> for HashTable<u32> {
  fn get_bucket(&self, key: &str) -> u64 {
    return hash(key) % self.n_buckets;
  }

  fn set(&mut self, key: &str, val: u32) {
    let bucket: u64 = self.get_bucket(key);
    println!("bucket: {}", bucket.to_string());
    let mut v = &mut self.buckets[bucket as usize];

    while ! v.is_none() {
      let t = v.as_mut();
      let item = t.unwrap();
      if item.key == key {
        item.val = val;
        return;
      }
      v = &mut item.next;
    }
    let mut entry = Entry{
      key: key.to_string(),
      val: val,
      next: None
    };
    //entry.next = Some(Box::new(**(self.buckets[bucket as usize].as_ref().unwrap())));
    mem::swap(&mut entry.next, &mut self.buckets[bucket as usize]);
   // entry.next = self.buckets[bucket as usize].to_owned();
    //let mut next = &mut entry.next;
    //next = &mut self.buckets[bucket as usize];
    if ! entry.next.is_none() {
     println!("entry.next:{:?}", entry.next.as_ref().unwrap().val);
    }
    //println!("{:?}", next.as_ref().unwrap().val);
    self.buckets[bucket as usize] = Some(Box::new(entry));
    if ! self.buckets[bucket as usize].is_none() {
      println!("first entry in bucket:{:?}", self.buckets[bucket as usize].as_ref().unwrap().val);
      if ! self.buckets[bucket as usize].as_ref().unwrap().next.is_none() {
        println!("second entry in bucket:{:?}", self.buckets[bucket as usize].as_ref().unwrap().next.as_ref().unwrap().val);
      }
    }
  }

  fn get(&mut self, key: &str) -> Option<u32> {
    let bucket: u64 = self.get_bucket(key);
    println!("bucket: {}", bucket.to_string());
    let mut v = & self.buckets[bucket as usize];

    while ! v.is_none() {
      let t = v.as_ref();
      let item = t.unwrap();
      if item.key == key {
        return Some(item.val);
      }
      v = & item.next;
    }
    return None;
  }

  fn delete(&mut self, key: &str) {
    let bucket: u64 = self.get_bucket(key);
    println!("bucket: {}", bucket.to_string());
    
    if self.buckets[bucket as usize].is_some() {
      let t = self.buckets[bucket as usize].as_ref();
      let item = t.unwrap();
      if item.key == key {
        println!("ohhhhhhhh");
        self.buckets[bucket as usize] = item.next.to_owned();
        return;
      }
    } else {
      return;
    }
    let mut current = self.buckets[bucket as usize].as_mut().unwrap();
    while current.next.is_some() {
      let item = current.next.as_mut().unwrap();
      if item.key == key {
        println!("ohhhhhhhh2");
        if let Some(next) = item.next.to_owned() {
          current.next = Some(Box::new(*next));
        } else {
          current.next  = None
        }
        return;
      } 
      current = current.next.as_mut().unwrap();
    }
  }
}

impl<T> HashTable<T> {
  fn new() -> HashTable<T> {
    return HashTable {
      n_buckets: 4,
      buckets: vec![None,None,None,None]
    };
  }
}


fn main() {
  println!("Hello, world!");
  let mut hash_map: HashTable<u32> = HashTable::new();
  let v = 5;
  hash_map.set("item a", v);
  hash_map.set("item a", 3);
  hash_map.set("item e", 6);
  hash_map.set("item i", 7);
  hash_map.set("item m", 9);
  println!("{:?}", hash_map.get("item a"));  
  println!("{:?}", hash_map.get("item e"));  
  println!("{:?}", hash_map.get("item i")); 
  println!("{:?}", hash_map.get("item m")); 
  println!("{:?}", hash_map.get("item z")); 
  println!("-------------");  
  hash_map.delete("item m");
  println!("{:?}", hash_map.get("item a"));  
  println!("{:?}", hash_map.get("item e"));  
  println!("{:?}", hash_map.get("item i")); 
  println!("{:?}", hash_map.get("item m")); 
  println!("{:?}", hash_map.get("item z")); 
  println!("-------------");  
  hash_map.delete("item e");
  println!("{:?}", hash_map.get("item a"));  
  println!("{:?}", hash_map.get("item e"));  
  println!("{:?}", hash_map.get("item i")); 
  println!("{:?}", hash_map.get("item m")); 
  println!("{:?}", hash_map.get("item z")); 
  hash_map.delete("item a");
  println!("{:?}", hash_map.get("item a"));  
  println!("{:?}", hash_map.get("item e"));  
  println!("{:?}", hash_map.get("item i")); 
  println!("{:?}", hash_map.get("item m")); 
  println!("{:?}", hash_map.get("item z")); 
  hash_map.delete("item z");
}