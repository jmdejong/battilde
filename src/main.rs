#![recursion_limit="512"]
use std::thread::sleep;
use std::time::Duration;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use structopt::StructOpt;
use chrono::Utc;
use std::fs;

mod server;
mod gameserver;
mod util;
mod controls;
mod worldmessages;
mod pos;
mod player;
mod world;
mod sprite;
mod timestamp;
mod config;
mod errors;
mod holder;
mod weapon;
mod creature;
mod tile;
mod item;
mod waves;
mod gamemode;
mod mapgen;
mod grid;

use self::{
	pos::{Pos, Direction},
	player::PlayerId,
	errors::{Result},
	sprite::Sprite,
	timestamp::Timestamp,
	
	gameserver::GameServer,
	server::Server,
	server::address::Address,
	controls::Action,
	world::World,
	worldmessages::MessageCache,
	mapgen::{MapType},
};



fn main(){
	
	let config = config::Config::from_args();
	
	println!("Server admin(s): {}", config.admins);
	
	let adresses = config.address
		.unwrap_or_else(||
			(if cfg!(target_os = "linux") {
				vec!["abstract:battilde", "inet:127.0.0.1:9221"]
			} else {
				vec!["inet:127.0.0.1:9221"]
			})
			.iter()
			.map(|a| a.parse().unwrap())
			.collect()
		);
	println!("adresses: {:?}", adresses);
	let servers: Vec<Box<dyn Server>> = 
		adresses
		.iter()
		.map(|a| a.to_server().unwrap())
		.collect();
	
	let mut gameserver = GameServer::new(servers, config.admins);
	
	let map = if let Some(map_path) = config.custom_map {
		let maptext = fs::read_to_string(&map_path).unwrap_or_else(|_| panic!("can't read map {:?}", map_path));
		let template = json5::from_str(&maptext).unwrap_or_else(|_| panic!("invalid map text:\n{:?}", maptext));
		MapType::Custom(template)
	} else {
		MapType::Builtin(config.map)
	};
	
	let mut world = World::new(config.game_mode, map);
	
	let mut message_cache = MessageCache::default();
	
	// close handler
	// todo: don't let the closing wait on sleep (using a timer thread or recv_timeout)
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
	ctrlc::set_handler(move || {
		println!("shutting down");
		r.store(false, Ordering::SeqCst);
	}).expect("can't set close handler");
	
	
	println!("battilde started on {}", Utc::now());
	
	let mut empty_timer = 1000000;
	
	while running.load(Ordering::SeqCst) {
		empty_timer += 1;
		let actions = gameserver.update();
		for action in actions {
			match action {
				Action::Input(player, control) => {
					if let Err(err) = world.control_player(player.clone(), control){
						println!("error controlling player {:?}: {:?}", player, err);
					}
				}
				Action::Join(player, sprite) => {
					if let Err(err) = world.add_player(&player, sprite) {
						println!("Error: can not add player {:?}: {:?}", player, err);
						if let Err(senderr) = gameserver.send_player_error(&player, "worlderror", "invalid room or savefile") {
							println!("Error: can not send error message to {:?}: {:?}", player, senderr);
						}
					}
				}
				Action::Leave(player) => {
					if let Err(err) = world.remove_player(&player) {
						println!("Error: can not remove player {:?}: {:?}", player, err);
					}
					message_cache.remove(&player);
					empty_timer = 0;
				}
			}
		}
		if world.nplayers() == 0 && empty_timer > 100 {
			if empty_timer == 600 {
				world.reset();
			}
			sleep(Duration::from_millis(500));
			continue;
		}
		world.update();
		let messages = world.view();
		for (player, mut message) in messages {
			message_cache.trim(&player, &mut message);
			if message.is_empty(){
				continue;
			}
// 			println!("m {}", message.to_json());
			if let Err(err) = gameserver.send(&player, message.to_json()) {
				println!("Error: failed to send to {:?}: {:?}", player, err);
			}
		}
		
		sleep(Duration::from_millis(config.step_duration));
	}
	println!("shutting down on {}", Utc::now());
}




