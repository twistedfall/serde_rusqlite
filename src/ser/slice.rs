extern crate rusqlite;

use super::{NamedSliceSerializer, PositionalSliceSerializer};
use std::borrow::Borrow;
use std::ops::{Deref, DerefMut};

/// Stores positional bound query arguments
///
/// This `struct` stores data for passing as argument slice to positional query functions of rusqlite.
/// To get the instance call crate's `to_params` function.
pub struct PositionalParamSlice(Vec<Box<rusqlite::types::ToSql>>);

impl PositionalParamSlice {
	pub fn to_slice(&self) -> Vec<&rusqlite::types::ToSql> {
		self.0.iter().map(|x| x.borrow()).collect()
	}
}

impl From<Vec<Box<rusqlite::types::ToSql>>> for PositionalParamSlice {
	fn from(src: Vec<Box<rusqlite::types::ToSql>>) -> Self {
		PositionalParamSlice(src)
	}
}

impl From<PositionalSliceSerializer> for PositionalParamSlice {
	fn from(src: PositionalSliceSerializer) -> Self {
		src.0
	}
}

impl Deref for PositionalParamSlice {
	type Target = Vec<Box<rusqlite::types::ToSql>>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl DerefMut for PositionalParamSlice {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

/// Stores named bound query arguments
///
/// This `struct` stores data for passing as argument slice to `*_named` query functions of rusqlite.
/// To get the instance call crate's `to_named_params` function.
pub struct NamedParamSlice(Vec<(String, Box<rusqlite::types::ToSql>)>);

impl NamedParamSlice {
	pub fn to_slice(&self) -> Vec<(&str, &rusqlite::types::ToSql)> {
		self.0.iter().map(|x| (x.0.as_str(), x.1.borrow())).collect()
	}
}

impl From<Vec<(String, Box<rusqlite::types::ToSql>)>> for NamedParamSlice {
	fn from(src: Vec<(String, Box<rusqlite::types::ToSql>)>) -> Self {
		NamedParamSlice(src)
	}
}

impl From<NamedSliceSerializer> for NamedParamSlice {
	fn from(src: NamedSliceSerializer) -> Self {
		src.0
	}
}

impl Deref for NamedParamSlice {
	type Target = Vec<(String, Box<rusqlite::types::ToSql>)>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl DerefMut for NamedParamSlice {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

