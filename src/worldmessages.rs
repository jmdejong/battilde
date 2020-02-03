
use serde_json::{Value, json};
use serde::Serialize;
use super::util::ToJson;
use super::pos::Pos;

#[derive(Clone)]
pub struct WorldMessage {
	pub updates: Vec<WorldUpdate>
}

impl ToJson for WorldMessage {
	fn to_json(&self) -> Value {
		let updates: Vec<Value> = self.updates.iter().map(|u| u.to_json()).collect();
		json!(["world", updates])
	}
}

#[derive(Clone)]
pub enum WorldUpdate {
	Field(FieldMessage),
	Pos(Pos),
	Change(Vec<(Pos, Vec<String>)>)
}

impl ToJson for WorldUpdate {
	fn to_json(&self) -> Value {
		match self {
			WorldUpdate::Field(msg) => json!(["field", msg]),
			WorldUpdate::Pos(pos) => json!(["playerpos", pos]),
			WorldUpdate::Change(changes) => json!(["changecells", changes])
		}
	}
}

#[derive(Clone, Serialize)]
pub struct FieldMessage {
	pub width: i32,
	pub height: i32,
	pub field: Vec<usize>,
	pub mapping: Vec<Vec<String>>
}



