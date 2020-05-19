
use std::collections::{HashSet, HashMap};
use std::hash::Hash;
use crate::{
	parameter::Parameter,
	Template,
	Pos,
	components::interactable::Interactable,
	PlayerId,
	Sprite,
	ItemId
};

pub trait FromToParameter: Sized {
	fn from_parameter(p: Parameter) -> Option<Self>;
	fn to_parameter(self) -> Parameter;
}



impl FromToParameter for Parameter {
	fn from_parameter(p: Parameter) -> Option<Self>{
		Some(p)
	}
	fn to_parameter(self) -> Parameter {
		self
	}
}

macro_rules! tofrom {
	($typ: ty : $paramtyp: ident) => {
		impl FromToParameter for $typ {
			fn from_parameter(p: Parameter) -> Option<Self>{
				if let Parameter::$paramtyp(i) = p {
					Some(i)
				} else {
					None
				}
			}
			fn to_parameter(self) -> Parameter {
				Parameter::$paramtyp(self)
			}
		}
	};
	($typ: ident { $arg: ident :  $paramtyp: ident } ) => {
		impl FromToParameter for $typ {
			fn from_parameter(p: Parameter) -> Option<Self>{
				if let Parameter::$paramtyp(i) = p {
					Some($typ { $arg: i})
				} else {
					None
				}
			}
			fn to_parameter(self) -> Parameter {
				Parameter::$paramtyp(self.$arg)
			}
		}
	};
	($typ: ident ($paramtyp: ident ) ) => {
		impl FromToParameter for $typ {
			fn from_parameter(p: Parameter) -> Option<Self>{
				if let Parameter::$paramtyp(i) = p {
					Some($typ (i))
				} else {
					None
				}
			}
			fn to_parameter(self) -> Parameter {
				Parameter::$paramtyp(self.0)
			}
		}
	}
}

tofrom!(i64: Int);
tofrom!(f64: Float);
tofrom!(bool:Bool);
tofrom!(String: String);
tofrom!(Pos: Pos);
tofrom!(Template: Template);
tofrom!(Interactable: Interaction);

tofrom!(PlayerId{name: String});
tofrom!(Sprite{name: String});
tofrom!(ItemId(String));

impl<T> FromToParameter for Vec<T>
where
	T: FromToParameter,
{
	fn from_parameter(p: Parameter) -> Option<Self>{
		if let Parameter::List(items) = p{
			let mut v = Self::new();
			for item in items {
				if let Some(t) = T::from_parameter(item){
					v.push(t);
				} else {
					return None;
				}
			}
			Some(v)
		} else {
			None
		}
	}
	fn to_parameter(self) -> Parameter {
		Parameter::List(self.into_iter().map(|item| item.to_parameter()).collect())
	}
}

impl<T> FromToParameter for HashSet<T>
where
	T: FromToParameter + Eq + Hash,
{
	fn from_parameter(p: Parameter) -> Option<Self>{
		Some(<Vec<T>>::from_parameter(p)?.into_iter().collect())
	}
	fn to_parameter(self) -> Parameter {
		self.into_iter().collect::<Vec<T>>().to_parameter()
	}
}

impl<T, U> FromToParameter for HashMap<T, U>
where
	T: FromToParameter + Eq + Hash,
	U: FromToParameter,
{
	fn from_parameter(p: Parameter) -> Option<Self>{
		Some(<Vec<(T, U)>>::from_parameter(p)?.into_iter().collect())
	}
	fn to_parameter(self) -> Parameter {
		self.into_iter().collect::<Vec<(T, U)>>().to_parameter()
	}
}


impl<T, U> FromToParameter for (T, U)
where
	T: FromToParameter,
	U: FromToParameter,
{
	fn from_parameter(p: Parameter) -> Option<Self> {
		if let Parameter::List(mut items) = p {
			if items.len() == 2 {
				return Some((
					T::from_parameter(items.remove(0))?,
					U::from_parameter(items.remove(0))?
				))
			}
		}
		None
	}
	fn to_parameter(self) -> Parameter {
		Parameter::List(vec![self.0.to_parameter(), self.1.to_parameter()])
	}
}

impl<T, U, V> FromToParameter for (T, U, V)
where
	T: FromToParameter,
	U: FromToParameter,
	V: FromToParameter,
{
	fn from_parameter(p: Parameter) -> Option<Self> {
		if let Parameter::List(mut items) = p {
			if items.len() == 3 {
				return Some((
					T::from_parameter(items.remove(0))?,
					U::from_parameter(items.remove(0))?,
					V::from_parameter(items.remove(0))?
				))
			}
		}
		None
	}
	fn to_parameter(self) -> Parameter {
		Parameter::List(vec![self.0.to_parameter(), self.1.to_parameter(), self.2.to_parameter()])
	}
}
