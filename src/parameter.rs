
use serde_json::{Value, json};
use crate::{
	Template,
	components::item::ItemAction,
	Pos
};



macro_rules! parameters {
	($($name: ident ($typ: ident) $stringname: ident, $v: ident ($fromjson: expr) ($tojson: expr));*;) => {
		#[derive(Debug, PartialEq, Clone)]
		pub enum Parameter {
			$(
				$name($typ),
			)*
		}
		impl Parameter {
			pub fn from_typed_json(typ: ParameterType, val: &Value) -> Option<Parameter>{
				match typ {
					$(
						ParameterType::$name => Some(Self::$name({
							let $v = val;
							$fromjson
						})),
					)*
				}
			}
			pub fn paramtype(&self) -> ParameterType {
				match self {
					$(
						Self::$name(_) => ParameterType::$name,
					)*
				}
			}
			pub fn to_json(&self) -> Value {
				match self {
					$(
						Self::$name($v) => $tojson,
					)*
				}
			}
		}

		#[derive(Debug, Clone, Copy, PartialEq, Eq)]
		pub enum ParameterType {
			$(
				$name,
			)*
		}
		impl ParameterType {
			pub fn from_str(typename: &str) -> Option<Self>{
				match typename {
					$(
						stringify!($stringname) => Some(Self::$name),
					)*
					_ => None
				}
			}
		}
	}
}

parameters!(
	String (String) string, v (v.as_str()?.to_string()) (json!(v));
	Int (i64) int, v (v.as_i64()?) (json!(v));
	Pos (Pos) pos, v (Pos::from_json(v)?) (json!(v));
	Float (f64) float, v (v.as_f64()?) (json!(v));
	Template (Template) template, v (Template::from_json(v).ok()?) (v.to_json());
	Action (ItemAction) action, v (ItemAction::from_json(v)?) (v.to_json());
	Bool (bool) bool, v (v.as_bool()?) (json!(v));
);


impl Parameter {
	#[allow(dead_code)]
	pub fn string(string: &str) -> Self {
		Self::String(string.to_string())
	}
	
	pub fn guess_from_json(val: &Value) -> Option<Parameter> {
		let typ = 
			if val.is_string() {
				ParameterType::String
			} else if val.is_u64() || val.is_i64() {
				ParameterType::Int
			} else if val.is_f64() {
				ParameterType::Float
			} else if val.is_boolean(){
				ParameterType::Bool
			} else if val.is_object(){
				ParameterType::Template
			} else {
				println!("{:?}", val);
				return None
			};
		Self::from_typed_json(typ, val)
	}
}



#[cfg(test)]
mod tests {
	use super::*;
	use serde_json::json;
	
	macro_rules! gfj { // guess from json
		($j:expr) => {Parameter::guess_from_json(&json!($j)).unwrap()}
	}
	
	#[test]
	fn can_guess_json() {
		Parameter::guess_from_json(&json!(3)).unwrap();
	}
	
	#[test]
	fn guess_json() {
		assert_eq!(gfj!("charles"), Parameter::string("charles"));
		assert_eq!(gfj!("1"), Parameter::string("1"));
		assert_eq!(gfj!(""), Parameter::string(""));
		assert_eq!(gfj!(3), Parameter::Int(3));
		assert_eq!(gfj!(-3), Parameter::Int(-3));
		assert_eq!(gfj!(0), Parameter::Int(0));
		assert_eq!(gfj!(-0), Parameter::Int(0));
		assert_eq!(gfj!(3.5), Parameter::Float(3.5));
		assert_eq!(gfj!(3.0), Parameter::Float(3.0));
		assert_eq!(gfj!(-3.0), Parameter::Float(-3.0));
		assert_eq!(gfj!(0.0), Parameter::Float(0.0));
		assert_eq!(gfj!(-0.0), Parameter::Float(0.0));
	}
	
	#[test]
	fn guess_json_none() {
		assert!(Parameter::guess_from_json(&json!([2, 5])).is_none());
		assert!(Parameter::guess_from_json(&json!(true)).is_none());
		assert!(Parameter::guess_from_json(&json!({"hello": "world"})).is_none());
	}
}
