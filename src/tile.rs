
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
pub enum WallType{
	Wall,
	Rubble,
	#[allow(dead_code)]
	Rock
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObstacleType{
	Water,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
	Floor(FloorType),
	Sanctuary,
	Gate,
	Wall(WallType),
	Obstacle(ObstacleType),
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
			Tile::Wall(WallType::Wall) => "wall",
			Tile::Wall(WallType::Rubble) => "rubble",
			Tile::Wall(WallType::Rock) => "rock",
			Tile::Obstacle(ObstacleType::Water) => "water"
		})
	}
	pub fn blocking(&self) -> bool {
		match self {
			Tile::Floor(_) => false,
			Tile::Sanctuary => false,
			Tile::Wall(_) => true,
			Tile::Gate => true,
			Tile::Obstacle(_) => true
		}
	}
	
	pub fn bullet_blocking(&self) -> bool {
		match self {
			Tile::Floor(_) => false,
			Tile::Sanctuary => false,
			Tile::Wall(_) => true,
			Tile::Gate => true,
			Tile::Obstacle(_) => false
		}
	}
	
	pub fn from_char(c: char) -> Option<Self>{
		Some(match c {
			'"' => Tile::Floor(FloorType::Stone),
			'.' => Tile::Floor(FloorType::Dirt),
			',' => Tile::Floor(FloorType::Grass1),
			'\'' => Tile::Floor(FloorType::Grass2),
			'`' => Tile::Floor(FloorType::Grass3),
			'=' => Tile::Gate,
			'+' => Tile::Sanctuary,
			'#' => Tile::Wall(WallType::Wall),
			'X' => Tile::Wall(WallType::Rock),
			'R' => Tile::Wall(WallType::Rubble),
			'~' => Tile::Obstacle(ObstacleType::Water),
			_ => {return None}
		})
	}
}
