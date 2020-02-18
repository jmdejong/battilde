
use std::path::PathBuf;
use std::fs;
use std::env;
use serde_json;
use serde_json::Value;
use crate::{
	PlayerId,
	RoomId,
	savestate::SaveState,
	playerstate::PlayerState,
	util::Result,
	aerr
};

pub trait PersistentStorage {
	
	fn load_room(&self, id: RoomId) -> Result<SaveState>;
	
	fn load_player(&self, id: PlayerId) -> Result<PlayerState>;
	
	fn save_room(&self, id: RoomId, state: SaveState) -> Result<()>;
	
	fn save_player(&self, id: PlayerId, sate: PlayerState) -> Result<()>;
	
}


pub struct FileStorage {
	directory: PathBuf
}

impl FileStorage {
	pub fn new(path: PathBuf) -> Self {
		Self {
			directory: path
		}
	}
	
	pub fn savedir() -> Option<PathBuf> {
		if let Some(pathname) = env::var_os("ASCIIFARM_SAVE_DIR") {
			Some(PathBuf::from(pathname))
		} else if let Some(pathname) = env::var_os("XDG_DATA_HOME") {
			let mut path = PathBuf::from(pathname);
			path.push("asciifarm");
			path.push("saves");
			Some(path)
		} else if let Some(pathname) = env::var_os("HOME") {
			let mut path = PathBuf::from(pathname);
			path.push(".asciifarm");
			path.push("saves");
			Some(path)
		} else {
			None
		}
	}
}

impl PersistentStorage for FileStorage {
	
	fn load_room(&self, id: RoomId) -> Result<SaveState> {
		let mut path = self.directory.clone();
		path.push("rooms");
		let fname = id.to_string() + ".save.json";
		path.push(fname);
		let text = fs::read_to_string(path)?;
		let json: Value = serde_json::from_str(&text)?;
		SaveState::from_json(&json).ok_or(aerr!("not a valid save state"))
	}
	
	fn load_player(&self, id: PlayerId) -> Result<PlayerState> {
		let mut path = self.directory.clone();
		path.push("players");
		let fname = id.to_string() + ".save.json";
		path.push(fname);
		let text = fs::read_to_string(path)?;
		let json: Value = serde_json::from_str(&text)?;
		PlayerState::from_json(&json).ok_or(aerr!("not a valid save state"))
	}
	
	fn save_room(&self, id: RoomId, state: SaveState) -> Result<()> {
		let mut path = self.directory.clone();
		path.push("rooms");
		fs::create_dir_all(&path)?;
		let fname = id.to_string() + ".save.json";
		path.push(fname);
		let text = state.to_json().to_string();
		// todo: write to a temp file first
		fs::write(path, text)?;
		Ok(())
	}
	
	fn save_player(&self, id: PlayerId, state: PlayerState) -> Result<()> {
		let mut path = self.directory.clone();
		path.push("players");
		fs::create_dir_all(&path)?;
		let fname = id.to_string() + ".save.json";
		path.push(fname);
		let text = state.to_json().to_string();
		// todo: write to a temp file first
		fs::write(path, text)?;
		Ok(())
	}
}

