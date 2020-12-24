
use crate::sprite::Sprite;

#[derive(Debug, Clone)]
pub enum Item {
	Health
}

impl Item {
	pub fn sprite(&self) -> Sprite {
		Sprite(match self {
			Self::Health => "health"
		})
	}
}
