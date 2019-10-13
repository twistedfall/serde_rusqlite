use std::{error::Error as StdError, fmt, result};

use serde::{de, ser};

#[derive(Debug)]
pub enum Error {
	/// This type of serialization or deserialization is not supported
	Unsupported(String),
	/// The value is too large, e.g. trying to serialize `u64` that is too large to fit in `i64`
	ValueTooLarge(String),
	/// General error during serialization
	Serialization(String),
	/// General error during deserialization
	Deserialization(String),
	/// Error originating from rusqlite
	Rusqlite(rusqlite::Error),
	/// No column name information available
	ColumnNamesNotAvailable,
}

pub type Result<T> = result::Result<T, Error>;

impl Error {
	/// Create the instance of `Unsupported` during serialization `Error`
	pub fn ser_unsupported(typ: &str) -> Self {
		Error::Unsupported(format!("Serialization is not supported from type: {}", typ))
	}

	/// Create the instance of `Unsupported` during deserialization `Error`
	pub fn de_unsupported(typ: &str) -> Self {
		Error::Unsupported(format!("Deserialization is not supported into type: {}", typ))
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
		match self {
			Error::Unsupported(s) | Error::ValueTooLarge(s) => write!(f, "{}", s),
			Error::Serialization(s) => write!(f, "Serialization error: {}", s),
			Error::Deserialization(s) => write!(f, "Deserialization error: {}", s),
			Error::Rusqlite(s) => write!(f, "Rusqlite error: {}", s),
			Error::ColumnNamesNotAvailable => write!(f, "Column names are not available"),
		}
	}
}

impl StdError for Error {
	fn source(&self) -> Option<&(dyn StdError + 'static)> {
		match self {
			Error::Rusqlite(e) => Some(e),
			_ => None,
		}
	}
}

impl de::Error for Error {
	fn custom<T: fmt::Display>(msg: T) -> Self {
		Error::Deserialization(msg.to_string())
	}
}

impl ser::Error for Error {
	fn custom<T: fmt::Display>(msg: T) -> Self {
		Error::Serialization(msg.to_string())
	}
}

impl From<rusqlite::Error> for Error {
	fn from(e: rusqlite::Error) -> Self {
		Error::Rusqlite(e)
	}
}
