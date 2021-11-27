

use std::fmt;
use serde::{Serialize, Serializer};

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub enum Sprite {
	Custom(&'static str),
	Letter(char),
	Player(&'static str, char)
}

const VALID_COLOURS: &'static[&'static str] = &["r", "g", "b", "c", "m", "y", "lr", "lg", "lb", "lc", "lm", "ly", "a"];

impl Sprite {
	
	pub const fn new(name: &'static str) -> Self {
		Self::Custom(name)
	}
	
	pub fn player_sprite(spritename: &str) -> Option<Sprite> {
		let lowername = spritename.to_lowercase();
		let (colour_name, letter_str) = lowername.strip_prefix("player_")?.split_once("-")?;
		let letter: char = letter_str.chars().next()?;
		let colour = VALID_COLOURS.iter().find(|colour| *colour == &colour_name)?;
		if letter_str.len() == 1 && letter.is_ascii_alphabetic() {
			Some(Self::Player(colour, letter))
		} else {
			None
		}
	}
	
	pub fn letter_sprite(letter: char) -> Option<Sprite> {
		if letter.is_ascii_graphic() {
			Some(Self::Letter(letter))
		} else {
			None
		}
	}
}




impl fmt::Display for Sprite {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Sprite::Custom(name) => write!(f, "{}", name),
			Sprite::Letter(letter) => write!(f, "emptyletter-{}", letter),
			Sprite::Player(colour, letter) => write!(f, "player_{}-{}", colour, letter)
		}
	}
}



impl Serialize for Sprite {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where S: Serializer {
		format!("{}", self).serialize(serializer)
	}
}


#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_player_sprite_creation() {
		assert_eq!(Sprite::player_sprite("player_lg-a"), Some(Sprite::Player("lg", 'a')));
	}
	#[test]
	fn test_player_sprite_display() {
		assert_eq!(format!("{}", Sprite::Player("lg", 'a')), "player_lg-a".to_string());
	}
	#[test]
	fn test_letter_sprite_creation() {
		assert_eq!(Sprite::letter_sprite('A'), Some(Sprite::Letter('A')));
	}
	#[test]
	fn test_letter_sprite_display() {
		assert_eq!(format!("{}", Sprite::Letter('A')), "emptyletter-A".to_string());
	}
}
