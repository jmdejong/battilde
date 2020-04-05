
use std::thread::sleep;
use std::time::Duration;
use std::path::PathBuf;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::fs;
use serde_json;
use structopt::StructOpt;

mod server;
mod gameserver;
mod room;
mod util;
mod controls;
mod components;
mod resources;
mod systems;
mod worldmessages;
mod pos;
mod componentwrapper;
mod parameter;
mod assemblage;
mod componentparameter;
mod encyclopedia;
mod template;
mod roomtemplate;
mod savestate;
mod playerid;
mod playerstate;
mod roomid;
mod persistence;
mod worldloader;
mod world;
mod sprite;
mod timestamp;
mod purgatory;
mod config;
mod item;

use self::{
	pos::Pos,
	playerid::PlayerId,
	roomid::RoomId,
	item::ItemId,
	util::Result,
	sprite::Sprite,
	template::Template,
	encyclopedia::Encyclopedia,
	timestamp::Timestamp,
	
	gameserver::GameServer,
	server::Server,
	server::address::Address,
	persistence::FileStorage,
	controls::Action,
	worldloader::WorldLoader,
	world::World,
	worldmessages::MessageCache
};



fn main() -> Result<()>{
	
	let config = config::Config::from_args();
	
	let adresses = config.address
		.unwrap_or(vec!["abstract:rustifarm".parse()?, "inet:127.0.0.1:1234".parse()?]);
	println!("adresses: {:?}", adresses);
	let servers: Vec<Box<dyn Server>> = 
		adresses
		.iter()
		.map(|a| a.to_server().unwrap())
		.collect();
	
	let mut gameserver = GameServer::new(servers);
	
	let content_dir = config.content_dir.unwrap_or(
		PathBuf::new()
			.join(std::env::var("CARGO_MANIFEST_DIR").unwrap_or(".".to_string()))
			.join("content/")
	);
	println!("content directory: {:?}", content_dir);
	let loader = WorldLoader::new(content_dir.join("maps"));
	let encyclopedia = Encyclopedia::from_json(
		serde_json::from_str(
			&fs::read_to_string(
				content_dir
					.join("encyclopediae")
					.join("default_encyclopedia.json")
			)?
		)?
	)?;
	
	let save_dir = config.save_dir.unwrap_or(
		FileStorage::default_save_dir().expect("couldn't find any save directory")
	);
	println!("save directory: {:?}", content_dir);
	let storage = FileStorage::new(save_dir);

	let mut world = World::new(encyclopedia, loader, Box::new(storage), RoomId::from_str("room"));
	
	let mut message_cache = MessageCache::default();
	
	// close handler
	// todo: don't let the closing wait on sleep (using a timer thread or recv_timeout)
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
	ctrlc::set_handler(move || {
		println!("shutting down");
		r.store(false, Ordering::SeqCst);
	}).expect("can't set close handler");
	
	
	println!("asciifarm started");
	
	
	let mut count = 0;
	while running.load(Ordering::SeqCst) {
		let actions = gameserver.update();
		for action in actions {
			match action {
				Action::Input(player, control) => {
					let _ = world.control_player(player, control);
				}
				Action::Join(player) => {
					world.add_player(&player)?;
				}
				Action::Leave(player) => {
					world.remove_player(&player)?;
					message_cache.remove(&player);
				}
			}
		}
		world.update();
		if count % 50 == 0 {
			world.save();
			world.unload_rooms();
		}
		let messages = world.view();
		for (player, mut message) in messages {
			message_cache.trim(&player, &mut message);
			if message.is_empty(){
				continue;
			}
// 			println!("m {}", message.to_json());
			let _ = gameserver.send(&player, message.to_json());
		}
		
		count += 1;
		sleep(Duration::from_millis(100));
	}
	println!("saving world");
	world.save();
	println!("world saved");
	Ok(())
}




