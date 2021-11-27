

use std::fmt;
use serde::{Serialize, Deserialize};
use battilde_macros::generate_player_sprites;

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq, Serialize, Deserialize)]
pub struct Sprite(pub &'static str);

macro_rules! makesprites {
	($prefix: expr, $($letter: tt)*) => {&[$(Sprite(concat!($prefix, stringify!($letter)))),*]};
}

impl Sprite {
	pub const PLAYER_SPRITES: &'static [Sprite] = generate_player_sprites!();
	pub const LETTERS: &'static [Sprite] = makesprites!("emptyletter-", A B C D E F G H I J K L M N O P Q R S T U V W X Y Z ~ ! @ # $ % ^ & * ( ) _ + - =);
	
	pub fn player_sprite(spritename: &str) -> Option<Sprite> {
		Sprite::PLAYER_SPRITES.iter().find(|s|s.0.to_lowercase() == spritename.to_lowercase()).cloned()
	}
	
	pub fn letter_sprite(letter: char) -> Option<Sprite> {
		let spritename = format!("emptyletter-{}", letter);
		Sprite::LETTERS.iter().find(|s|s.0 == spritename).cloned()
	}
}

impl fmt::Display for Sprite {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.0)
	}
}
