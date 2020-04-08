
use std::collections::HashMap;
use serde_json::{json, Value};
use crate::{
	Pos,
	Template,
	PResult,
	perr
};

pub struct SaveState {
	pub changes: HashMap<Pos, Vec<Template>>
}

impl SaveState {
	
	pub fn new() -> Self {
		Self {
			changes: HashMap::new()
		}
	}
	
	pub fn to_json(&self) -> Value {
		json!({
			"changes": self.changes.iter().map(
					|(pos, templates)|
					(pos, templates.iter().map(|t| t.to_json()).collect())
				).collect::<Vec<(&Pos, Vec<Value>)>>()
		})
	}
	
	pub fn from_json(val: &Value) -> PResult<Self> {
		let mut changes = HashMap::new();
		for v in val.get("changes").ok_or(perr!("save does not have changes"))?.as_array().ok_or(perr!("changes not an array"))? {
			let pos = Pos::from_json(v.get(0).ok_or(perr!("change does not have index 0"))?).ok_or(perr!("change index 0 is not a pos"))?;
			let mut templates = Vec::new();
			let jsontemplates = v.get(1).ok_or(perr!("change does not have index 1"))?;
			for t in jsontemplates.as_array().clone().unwrap_or(&vec![jsontemplates.clone()]) {
				templates.push(Template::from_json(t)?);
			}
			changes.entry(pos).or_insert_with(Vec::new).append(&mut templates);
		}
		Ok(Self {changes})
	}
}
