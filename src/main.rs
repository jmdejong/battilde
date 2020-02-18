
use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;
use std::path::Path;

use serde_json::json;

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
mod defaultencyclopedia;
mod playerstate;
mod roomid;
mod persistence;

pub use self::pos::Pos;
pub use self::playerid::PlayerId;
pub use self::roomid::RoomId;

use self::gameserver::GameServer;
use self::server::unixserver::UnixServer;
use self::server::tcpserver::TcpServer;
use self::server::Server;
use self::room::Room;
use self::util::ToJson;
use self::roomtemplate::RoomTemplate;
use self::defaultencyclopedia::default_encyclopedia;
use self::persistence::{FileStorage, PersistentStorage};
use crate::controls::Action;
use crate::playerstate::PlayerState;



fn main() {
	
	let mut servers: Vec<Box<dyn Server>> = Vec::new();

	let addr = Path::new("\0rustifarm");
	let unixserver = UnixServer::new(&addr).expect("binding unix server failed");
	servers.push(Box::new(unixserver));
	
	let addr = "127.0.0.1:1234".parse().unwrap();
	let inetserver = TcpServer::new(&addr).expect("binding inet server failed");
	servers.push(Box::new(inetserver));
	
	let mut gameserver = GameServer::new(servers);
	
	let mut room = gen_room();
	
	let storage = FileStorage::new(FileStorage::savedir().expect("couldn't find any save directory"));
	if let Ok(state) = storage.load_room(RoomId::from_str("room")) {
		room.load_saved(&state);
		println!("loaded saved state successfully");
	} else {
		println!("loading saved state failed");
	}
	
	println!("asciifarm started");
	
	
	let mut count = 0;
	loop {
		let actions = gameserver.update();
		let mut inputs = HashMap::new();
		for action in actions {
			match action {
				Action::Input(player, control) => {inputs.insert(player, control);}
				Action::Join(player) => {
					let state = match storage.load_player(player.clone()) {
						Ok(state) => state,
						Err(_) => PlayerState::new(player.clone())
					};
					room.add_player(&state);
				}
				Action::Leave(player) => {
					if let Err(err) = storage.save_player(player.clone(), room.remove_player(player).unwrap()) {
						println!("{:?}", err);
					}
				}
			}
		}
		room.set_input(inputs);
		room.update();
		if count % 50 == 0 {
			if let Err(err) = storage.save_room(room.id.clone(), room.save()) {
				println!("{:?}",err);
			} else {
				println!("{}", room.save().to_json());
			}
			for (playerid, state) in room.save_players() {
				if let Err(err) = storage.save_player(playerid.clone(), state.clone()) {
					println!("{:?}",err);
				} else {
					println!("{:?} {}", playerid, state.to_json());
				}
			}
		}
		let messages = room.view();
		for (player, message) in messages {
			let _ = gameserver.send(&player, message.to_json());
		}
		
		count += 1;
		sleep(Duration::from_millis(100));
	}
}

fn gen_room<'a, 'b>() -> Room<'a, 'b> {
	let assemblages = default_encyclopedia();
	let mut room = Room::new(RoomId::from_str("room"), assemblages);

	let roomtemplate = RoomTemplate::from_json(&json!({
		"width": 42,
		"height": 22,
		"spawn": [5, 15],
		"field": [
			"     XXXXXXXXXXXX~~~XXXXXXXXXXXXXXXXXXXXXX",
			"     ,,,,,,,,,,,,~~~,,,,,,,,,,,,,,,,,,,,,X",
			"    ,,,,,,,,,,,,,~~~,,,,,,,,,,,,,,,,,,,,,X",
			"    ,,,,,,,,,,,,,~~~~,,,,,,,,,,,,,,,,,,,,X",
			" bbbb..,,,,,,,,,,,~~~,,,,,,,,,,,,,,,,,,,,X",
			"    ,,.,,,,,,,,,,,~~~,,,,,,,,,,,,,,,,,,,,X",
			"   ,,,.,,,,,,,,,,,~~~,,,,,,,,,,,,,,,,,,,,X",
			"  ,,,,.,,,,,,,,,,,~~~,,,,,,,,,,,,,,,,,,,,X",
			"X,,,,,.,,,,,,,,,,,~~~~,,,,,,T,,,,,,,,,,,,X",
			"X,,,,,.,,,,,,,,,,,,~~~,,,,,,,,,,,,,,,,,,,X",
			"X,,,,,.,,,,,,,,,,,,~~~,,,,,T,,,,######,,,X",
			"X,,,,,.,,,,,,,,,,,,bbb,,,,,,,,,,#++++#,,,X",
			"X,,,,,.............bbb...........++++#,,,X",
			"X,**,,.,,,,,,,,,,,,bbb,,,,,,,,,,#++++#,,,X",
			"X,*,*,.,,,,,,,,,,,,~~~,,,T,,,T,,#++++#,,,X",
			"X,,*,,.,,,,,,,,,,,,~~~,,,,,,,,,,######,,,X",
			"X,**,,.,,,,,,,,,,,~~~~,,,,,,,,,,f,,,,f,,,X",
			"X,,*,,.,,,,,,,,,,,~~~''''''''''''''''f'''X",
			"X*,,,,.,,,,,,,,,,,~~~'''''''''''f''''f'''X",
			"X,,,,,.,,,,,,,,,,,~~~'''''''''''ffffff'''X",
			"X,,,,,.,,,,,,,,,,,~~~''''''''''''''''''''X",
			"XXXXX,.,XXXXXXXXXX~~~XXXXXXXXXXXXXXXXXXXXX"
		],
		"mapping": {
			"#": "wall",
			",": "grass",
			".": "ground",
			"~": "water",
			"b": "bridge",
			"+": "floor",
			"'": "greengrass",
			"T": ["grass", "tree"],
			"f": ["grass", "fence"],
			"X": "rock",
			"*": ["grass", "pebble"],
			" ": []
		}
	})).unwrap();
	room.load_from_template(&roomtemplate);
	room
}


