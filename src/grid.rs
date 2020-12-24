
use crate::Pos;

#[derive(Debug, Clone)]
pub struct Grid<T> {
	size: Pos,
	storage: Vec<T>
}

impl<T: Clone> Grid<T> {
	
	pub fn empty() -> Grid<T> {
		Self {
			size: Pos::new(0, 0),
			storage: Vec::new()
		}
	}
	
	pub fn new(size: Pos, filler: T) -> Grid<T> {
		let mut storage = Vec::with_capacity((size.x * size.y) as usize);
		storage.resize((size.x * size.y) as usize, filler);
		Self {
			size,
			storage,
		}
	}
	
	#[inline]
	pub fn set_unchecked(&mut self, pos: Pos, val: T){
		self.storage[(pos.x + self.size.x * pos.y) as usize] = val;
	}
	
	#[inline]
	pub fn get_unchecked(&self, pos: Pos) -> &T{
		&self.storage[(pos.x + self.size.x * pos.y) as usize]
	}
	#[allow(dead_code)]
	#[inline]
	pub fn get_mut_unchecked(&mut self, pos: Pos) -> &T{
		&mut self.storage[(pos.x + self.size.x * pos.y) as usize]
	}
	
	#[inline]
	fn index(&self, pos: Pos) -> Option<usize> {
		if pos.x >= 0 && pos.y >= 0 && pos.x < self.size.x && pos.y < self.size.y {
			Some((pos.x + self.size.x * pos.y) as usize)
		} else {
			None
		}
	}
	
	#[inline]
	pub fn get(&self, pos: Pos) -> Option<&T>{
		Some(&self.storage[self.index(pos)?])
	}
	
	#[allow(dead_code)]
	#[inline]
	pub fn get_mut(&mut self, pos: Pos) -> Option<&mut T>{
		let index = self.index(pos)?;
		Some(&mut self.storage[index])
	}
	
	#[inline]
	pub fn set(&mut self, pos: Pos, val: T) -> Option<()>{
		let index = self.index(pos)?;
		self.storage[index] = val;
		Some(())
	}
}

