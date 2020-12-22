

use std::ops::{Add, Sub, Neg};
use serde::{Serialize, Serializer, Deserialize, Deserializer};
use crate::util::clamp;



#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all="lowercase")]
pub enum Direction {
	North,
	South,
	East,
	West
}

impl Direction {
	
	pub fn to_position(self) -> Pos {
		match self {
			Direction::North => Pos::new(0, -1),
			Direction::South => Pos::new(0, 1),
			Direction::East => Pos::new(1, 0),
			Direction::West => Pos::new(-1, 0)
		}
	}
	
	pub const DIRECTIONS: [Direction; 4] = [Direction::North, Direction::South, Direction::East, Direction::West];
}


#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, Default)]
pub struct Pos {
	pub x: i64,
	pub y: i64
}


impl Pos {
	
	pub fn new(x: i64, y: i64) -> Pos {
		Pos {x, y}
	}
	
	#[allow(dead_code)]
	pub fn from_tuple(p: (i64, i64)) -> Pos {
		let (x, y) = p;
		Pos {x, y}
	}
	
	#[allow(dead_code)]
	pub fn clamp(self, smaller: Pos, larger: Pos) -> Pos {
		Pos {
			x: clamp(self.x, smaller.x, larger.x),
			y: clamp(self.y, smaller.y, larger.y)
		}
	}
	
	pub fn abs(self) -> Pos {
		Pos{x: self.x.abs(), y: self.y.abs()}
	}
	
	pub fn size(&self) -> i64 {
		self.x.abs() + self.y.abs()
	}
	
	pub fn distance_to(&self, other: Pos) -> i64 {
		(other - *self).size()
	}
	
	pub fn directions_to(&self, other: Pos) -> Vec<Direction> {
		let mut directions = Vec::new();
		let d = other - *self;
		if d.x > 0 {
			directions.push(Direction::East);
		}
		if d.x < 0 {
			directions.push(Direction::West);
		}
		if d.y > 0 {
			directions.push(Direction::South);
		}
		if d.y < 0 {
			directions.push(Direction::North);
		}
		directions
	}
}


impl Serialize for Pos {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where S: Serializer {
		(self.x, self.y).serialize(serializer)
	}
}
impl<'de> Deserialize<'de> for Pos {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where D: Deserializer<'de> {
		let (x, y) = <(i64, i64)>::deserialize(deserializer)?;
		Ok(Self{x, y})
	}
}


impl Add<Pos> for Pos {
	type Output = Pos;
	fn add(self, other: Pos) -> Pos {
		Pos {
			x: self.x + other.x,
			y: self.y + other.y
		}
	}
}

impl Add<Direction> for Pos {
	type Output = Pos;
	fn add(self, dir: Direction) -> Pos {
		let other = dir.to_position();
		Pos {
			x: self.x + other.x,
			y: self.y + other.y
		}
	}
}

impl Sub<Pos> for Pos {
	type Output = Pos;
	fn sub(self, other: Pos) -> Pos {
		Pos {
			x: self.x - other.x,
			y: self.y - other.y
		}
	}
}

impl Neg for Pos {
    type Output = Pos;
    fn neg(self) -> Pos {
		Pos {x: -self.x, y: -self.y}
    }
}



