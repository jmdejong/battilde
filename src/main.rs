
use std::thread::sleep;
use std::time::Duration;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use structopt::StructOpt;
use chrono::Utc;

mod server;
mod gameserver;
mod util;
mod controls;
mod worldmessages;
mod pos;
mod playerid;
mod world;
mod sprite;
mod timestamp;
mod config;
mod errors;
mod auth;

use self::{
	pos::Pos,
	playerid::PlayerId,
	errors::{Result},
	sprite::Sprite,
	timestamp::Timestamp,
	
	gameserver::GameServer,
	server::Server,
	server::address::Address,
	controls::Action,
	world::World,
	worldmessages::MessageCache
};



fn main(){
	
	let config = config::Config::from_args();
	
	println!("Server admin(s): {}", config.admins);
	
	let adresses = config.address
		.unwrap_or(
			(if cfg!(target_os = "linux") {
				vec!["abstract:battilde", "inet:127.0.0.1:9021"]
			} else {
				vec!["inet:127.0.0.1:9021"]
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
	
	let user_dir = config.user_dir.unwrap_or(
		auth::FileRegister::default_register_dir().expect("couldn't find any save directory")
	);
	println!("user auth directory: {:?}", user_dir);
	let users = auth::FileRegister::new(user_dir);
	
	let mut gameserver = GameServer::new(servers, Box::new(users), config.admins);
	


	let mut world = World::new();
	
	let mut message_cache = MessageCache::default();
	
	// close handler
	// todo: don't let the closing wait on sleep (using a timer thread or recv_timeout)
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
	ctrlc::set_handler(move || {
		println!("shutting down");
		r.store(false, Ordering::SeqCst);
	}).expect("can't set close handler");
	
	
	println!("asciifarm started on {}", Utc::now());
	
	
	while running.load(Ordering::SeqCst) {
		let actions = gameserver.update();
		for action in actions {
			match action {
				Action::Input(player, control) => {
					if let Err(err) = world.control_player(player.clone(), control){
						println!("error controlling player {:?}: {:?}", player, err);
					}
				}
				Action::Join(player) => {
					if let Err(err) = world.add_player(&player) {
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
				}
			}
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




