

use std::fmt;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq, Serialize, Deserialize)]
pub struct Sprite(pub &'static str);

macro_rules! makeplayersprites {
	($l_old:ident ($l:ident $($ll:tt)*); () ($($t_all:tt)*); [$($out:tt)*]) 
		=> {makeplayersprites!($l ($($ll)*); ($($t_all)*) ($($t_all)*); [$($out)*])};
	($l:ident ($($ll:tt)*); ($t:ident $($ts:tt)*) ($($t_all:tt)*); [$($out:tt)*])
		=> {makeplayersprites!($l ($($ll)*); ($($ts)*) ($($t_all)*); [$($out)* (Sprite(concat!("player_", stringify!($l), '-' ,stringify!($t))))])};
	($l_old:ident (); () ($($t_all:tt)*); [$(($($out:tt)*))*]) => {&[$($($out)*),*]};
}

macro_rules! makesprites {
	($prefix: expr, $($letter: tt)*) => {&[$(Sprite(concat!($prefix, stringify!($letter)))),*]};
}

impl Sprite {
	pub const PLAYER_SPRITES: &'static [Sprite] = 
		makeplayersprites!(old (r g b c m y lr lg lb lc lm ly a); () (A B C D E F G H I J K L M N O P Q R S T U V W X Y Z); []);
	pub const LETTERS: &'static [Sprite] = makesprites!("emptyletter-", A B C D E F G H I J K L M N O P Q R S T U V W X Y Z ~ ! @ # $ % ^ & * ( ) _ + - =);
	
	pub fn player_sprite(spritename: &str) -> Option<Sprite> {
		Sprite::PLAYER_SPRITES.iter().find(|s|s.0 == spritename).cloned()
	}
	
	pub fn letter_sprite(letter: char) -> Option<Sprite> {
		let spritename = format!("emptyletter-{}", letter);
		Sprite::LETTERS.iter().find(|s|s.0 == &spritename).cloned()
	}
}


impl fmt::Display for Sprite {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.0)
	}
}
