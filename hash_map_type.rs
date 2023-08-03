#[derive(Clone)]
pub struct Entry<T> {
  pub key: String,
  pub val: T,
  pub next: Option<Box<Entry<T>>>
}

pub struct HashTable<T> {
  pub n_buckets: u64,
  pub buckets: Vec<Option<Box<Entry<T>>>>
}

pub fn hash(s: &str) -> u64 {
  let mut h: u64 = 5381;

  for ch in s.chars() { 
    h = 33*h + ch as u64
  }
  return h;
}

pub trait HashTableTrait<T> {
  fn get_bucket(&self, key: &str) -> u64;
  fn set(&mut self, key: &str, val: T);
  fn get(&mut self, key: &str) -> Option<T>;
  fn delete(&mut self, key: &str);
}