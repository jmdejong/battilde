
use std::str::FromStr;
use crate::{
	aerr,
	errors::AnyError
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameMode {
	Cooperative,
	PvP
}

impl FromStr for GameMode {
	type Err = AnyError;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"coop" => Ok(Self::Cooperative),
			"pvp" => Ok(Self::PvP),
			_ => Err(aerr!("'{}' is not a valid gamemode", s))
		}
	}
}
