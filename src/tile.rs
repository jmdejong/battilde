
use crate::Sprite;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FloorType{
	#[allow(dead_code)]
	Stone,
	Dirt,
	Grass1,
	Grass2,
	Grass3
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
	Floor(FloorType),
	Sanctuary,
	Gate,
	Wall
}

impl Tile {
	pub fn sprite(&self) -> Sprite{
		Sprite(match self {
			Tile::Floor(FloorType::Stone) => "floor",
			Tile::Floor(FloorType::Dirt) => "ground",
			Tile::Floor(FloorType::Grass1) => "grass1",
			Tile::Floor(FloorType::Grass2) => "grass2",
			Tile::Floor(FloorType::Grass3) => "grass3",
			Tile::Gate => "gate",
			Tile::Sanctuary => "sanctuary",
			Tile::Wall => "wall"
		}.to_string())
	}
	pub fn blocking(&self) -> bool {
		match self {
			Tile::Floor(_) => false,
			Tile::Sanctuary => false,
			Tile::Wall => true,
			Tile::Gate => true
		}
	}
}
