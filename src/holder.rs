
use std::collections::{HashMap, hash_map::{Iter, IterMut, Keys, Values}};

pub struct Holder<T> {
	counter: usize,
	storage: HashMap<usize, T>
}

impl<T> Holder<T> {
	
	pub fn new() -> Holder<T> {
		Self {
			counter: 1,
			storage: HashMap::new()
		}
	}
	
	pub fn insert(&mut self, val: T) -> usize {
		self.counter += 1;
		self.storage.insert(self.counter, val);
		self.counter
	}
	
	pub fn remove(&mut self, key: &usize) -> Option<T> {
		self.storage.remove(key)
	}
	
	pub fn get(&self, key: &usize) -> Option<&T> {
		self.storage.get(key)
	}
	
	pub fn get_mut(&mut self, key: &usize) -> Option<&mut T> {
		self.storage.get_mut(key)
	}
	
	pub fn iter(&self) -> Iter<usize, T> {
		self.storage.iter()
	}
	
	pub fn iter_mut(&mut self) -> IterMut<usize, T> {
		self.storage.iter_mut()
	}
	
	#[allow(dead_code)]
	pub fn keys(&self) -> Keys<usize, T> {
		self.storage.keys()
	}
	
	pub fn values(&self) -> Values<usize, T> {
		self.storage.values()
	}
	
	pub fn len(&self) -> usize {
		self.storage.len()
	}
	
	pub fn contains_key(&self, key: &usize) -> bool {
		self.storage.contains_key(key)
	}
}
