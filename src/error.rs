extern crate rusqlite;
extern crate serde;

use self::serde::{de, ser};
use std::{fmt, result};

error_chain! {
	types {
		Error, ErrorKind, ResultExt;
	}

	foreign_links {
		RuSQLiteError(rusqlite::Error) #[doc = "`rusqlite` error"];
	}

	errors {
		#[doc = "this type of serialization or deserialization is not supported"]
		Unsupported(err: String) {}
		#[doc = "the value is too large, e.g. trying to serialize `u64` that is too large to fit in `i64`"]
		ValueTooLarge(err: String) {}
	}
}

pub type Result<T> = result::Result<T, Error>;

impl Error {
	fn unsupported(err: &str) -> Error {
		ErrorKind::Unsupported(err.into()).into()
	}

	/// Create the instance of `Unsupported` during serialization `Error`
	pub fn ser_unsupported(typ: &str) -> Error {
		Error::unsupported(&format!("Serialization is not supported from type: {}", typ))
	}

	/// Create the instance of `Unsupported` during deserialization `Error`
	pub fn de_unsupported(typ: &str) -> Error {
		Error::unsupported(&format!("Deserialization is not supported into type: {}", typ))
	}
}

impl de::Error for Error {
	fn custom<T: fmt::Display>(msg: T) -> Self {
		msg.to_string().into()
	}
}

impl ser::Error for Error {
	fn custom<T: fmt::Display>(msg: T) -> Self {
		msg.to_string().into()
	}
}
