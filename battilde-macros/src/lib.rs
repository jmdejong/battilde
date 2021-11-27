
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn generate_player_sprites(_args: TokenStream) -> TokenStream {
	
	let letters = "abcdefghijklmnopqrstxyz@";
	let colours = "r g b c m y lr lg lb lc lm ly a";
	let mut sprites = Vec::new();
	
	for colour in colours.split(" ") {
		for letter in letters.chars() {
			let spritename = format!("player_{}-{}", colour, letter);
			sprites.push(quote!(Sprite(#spritename)));
		}
	}
	
	TokenStream::from(quote!(
		&[#(#sprites),*]
	))
}
