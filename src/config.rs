
use structopt::StructOpt;
use std::path::PathBuf;
use crate::{
	Address,
	gamemode::GameMode,
	mapgen::BuiltinMap
};

#[derive(Debug, StructOpt)]
#[structopt(name = "Battilde", about = "Multiplayer terminal shooter (server)")]
pub struct Config {
	
	#[structopt(short, long, help="A server type and address. Allowed server types: 'inet', 'unix', 'abstract'. Example: \"inet:127.0.0.1:1234\" or \"abstract:battilde\" or \"unix:/tmp/battilde\" or \"inet:[::1]:1234\"")]
	pub address: Option<Vec<Address>>,
	
	#[structopt(long, env="USER", help="The name(s) of the server admin(s)")]
	pub admins: String,
	
	#[structopt(long, default_value="100", help="The time (in milliseconds) between two steps")]
	pub step_duration: u64,
	
	#[structopt(long, default_value="survival", help="The gamemode of the server. Options: survival, pillars, pvp")]
	pub game_mode: GameMode,
	
	#[structopt(long, default_value="square", help="The built-in map to play. Ignored if --custom-map is used.")]
	pub map: BuiltinMap,
	
	#[structopt(long, help="File path for a custom map to play")]
	pub custom_map: Option<PathBuf>,
	
	
}
