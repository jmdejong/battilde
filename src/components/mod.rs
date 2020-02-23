
pub mod item;

pub use item::Item;

use specs::{
	DenseVecStorage,
	VecStorage,
	HashMapStorage,
	FlaggedStorage,
	Component
};

use crate::{
	Pos,
	PlayerId,
	RoomId,
	Sprite,
	controls::Control,
	template::Template,
	playerstate::RoomPos
};

#[derive(Debug, Clone)]
pub struct Position{
	pub pos: Pos
}
impl Position {
	pub fn new(pos: Pos) -> Position {
		Position{pos}
	}
}

impl Component for Position {
	type Storage = FlaggedStorage<Self, VecStorage<Self>>;
}

#[derive(Debug, Clone)]
pub struct Visible {
	pub sprite: Sprite,
	pub height: f64,
	pub name: String
}
impl Component for Visible {
	type Storage = FlaggedStorage<Self, VecStorage<Self>>;
}

#[derive(Component, Debug)]
pub struct Controller(pub Control);

#[derive(Component, Debug, Clone)]
pub struct Blocking;

#[derive(Component, Debug, Clone)]
pub struct Floor;

#[derive(Component, Debug, Clone)]
pub struct New;

#[derive(Component, Debug, Clone)]
pub struct Removed;

#[derive(Component, Debug, Clone)]
pub struct Moved {
	pub from: Pos
}

#[derive(Component, Debug, Clone)]
#[storage(HashMapStorage)]
pub struct Player {
	pub id: PlayerId
}
impl Player {
	pub fn new(id: PlayerId) -> Self {
		Self{id}
	}
}

#[derive(Debug, Clone, Default)]
pub struct Inventory {
	pub items: Vec<Item>,
	pub capacity: usize
}
impl Component for Inventory {
	type Storage = FlaggedStorage<Self, HashMapStorage<Self>>;
}

#[derive(Component, Debug, Clone)]
pub struct Health {
	pub health: i64,
	pub maxhealth: i64
}
impl Health {
	pub fn heal(&mut self, amount: i64) {
		self.health += amount;
		if self.health > self.maxhealth {
			self.health = self.maxhealth;
		}
	}
}

#[derive(Component, Debug, Clone)]
pub struct Serialise {
	pub template: Template
}

#[derive(Component, Debug, Clone)]
pub struct RoomExit {
	pub destination: RoomId,
	pub dest_pos: RoomPos
}
