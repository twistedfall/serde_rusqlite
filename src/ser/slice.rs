use std::borrow::Borrow;
use std::ops::{Deref, DerefMut};

/// Stores named bound query arguments
///
/// This `struct` stores data for passing as argument slice to `*_named()` query functions of rusqlite.
/// To get the instance call crate's `to_named_params()` function.
#[derive(Default)]
pub struct NamedParamSlice(Vec<(String, Box<dyn rusqlite::types::ToSql>)>);

impl NamedParamSlice {
	/// Converts the named bound query arguments to a [Vec] suitable for passing to `rusqlite` `*_named()` query functions
	///
	/// Call [Vec::as_slice()] or do a `&*` on the result to get the slice.
	pub fn to_slice(&self) -> Vec<(&str, &dyn rusqlite::types::ToSql)> {
		self.0.iter().map(|x| (x.0.as_str(), x.1.borrow())).collect()
	}
}

impl From<Vec<(String, Box<dyn rusqlite::types::ToSql>)>> for NamedParamSlice {
	fn from(src: Vec<(String, Box<dyn rusqlite::types::ToSql>)>) -> Self {
		Self(src)
	}
}

impl Deref for NamedParamSlice {
	type Target = Vec<(String, Box<dyn rusqlite::types::ToSql>)>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl DerefMut for NamedParamSlice {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}
