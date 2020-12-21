
use std::collections::{HashMap, hash_map::{Iter, IterMut, Keys, Values}};


/** A hashmap that assigns unique keys to each inserted value by itself */
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
	
	#[inline]
	pub fn remove(&mut self, key: &usize) -> Option<T> {
		self.storage.remove(key)
	}
	
	#[inline]
	pub fn get(&self, key: &usize) -> Option<&T> {
		self.storage.get(key)
	}
	
	#[inline]
	pub fn get_mut(&mut self, key: &usize) -> Option<&mut T> {
		self.storage.get_mut(key)
	}
	
	#[inline]
	pub fn iter(&self) -> Iter<usize, T> {
		self.storage.iter()
	}
	
	#[inline]
	pub fn iter_mut(&mut self) -> IterMut<usize, T> {
		self.storage.iter_mut()
	}
	
	#[allow(dead_code)]
	#[inline]
	pub fn keys(&self) -> Keys<usize, T> {
		self.storage.keys()
	}
	
	#[inline]
	pub fn values(&self) -> Values<usize, T> {
		self.storage.values()
	}
	
	#[inline]
	pub fn len(&self) -> usize {
		self.storage.len()
	}
	
	#[inline]
	pub fn contains_key(&self, key: &usize) -> bool {
		self.storage.contains_key(key)
	}
	
	#[inline]
	pub fn retain<F>(&mut self, f: F)
	where
		F: FnMut(&usize, &mut T) -> bool,
	{
		self.storage.retain(f)
	}
}
