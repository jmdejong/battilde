
use std::str::FromStr;
use crate::{
	aerr,
	errors::AnyError
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameMode {
	PillarDefence,
	Survival,
	PvP
}

impl GameMode {
	
	pub fn has_pillars(self) -> bool {
		self == Self::PillarDefence
	}
}

impl FromStr for GameMode {
	type Err = AnyError;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"coop" => Ok(Self::Survival),
			"survival" => Ok(Self::Survival),
			"pillars" => Ok(Self::PillarDefence),
			"pvp" => Ok(Self::PvP),
			_ => Err(aerr!("'{}' is not a valid gamemode", s))
		}
	}
}
