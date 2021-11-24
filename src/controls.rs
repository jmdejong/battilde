

use serde::{Serialize, Deserialize};
use crate::{PlayerId, Direction, Pos, sprite::Sprite};


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all="lowercase")]
pub enum Control {
	Move(Direction),
	Shoot(Option<Direction>),
	ShootPrecise(Pos),
	Suicide,
	NextWeapon,
	PreviousWeapon
}

#[derive(Debug, Clone)]
pub enum Action {
	Join(PlayerId, Sprite),
	Leave(PlayerId),
	Input(PlayerId, Control)
}

