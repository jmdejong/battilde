

use serde::{Serialize, Deserialize};
use crate::{PlayerId, Direction, sprite::Sprite};


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all="lowercase")]
pub enum Control {
	Move(Direction),
	Shoot(Option<Direction>)
}

#[derive(Debug, Clone)]
pub enum Action {
	Join(PlayerId, Sprite),
	Leave(PlayerId),
	Input(PlayerId, Control)
}

