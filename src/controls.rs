

use serde::{Serialize, Deserialize};
use crate::{PlayerId, Pos};

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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all="lowercase")]
pub enum Control {
	Move(Direction),
	Shoot(Option<Direction>)
}

#[derive(Debug, Clone)]
pub enum Action {
	Join(PlayerId),
	Leave(PlayerId),
	Input(PlayerId, Control)
}

