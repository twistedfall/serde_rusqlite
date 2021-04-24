use crate::{Error, PositionalParamSlice, Result};

use super::tosql::ToSqlSerializer;

use serde::ser;

macro_rules! forward_tosql {
	($fun:ident, $type:ty) => {
		fn $fun(mut self, v: $type) -> Result<Self::Ok> {
			self.0.push(ToSqlSerializer.$fun(v)?);
			Ok(self.0)
		}
	};
	($fun:ident) => {
		fn $fun(mut self) -> Result<Self::Ok> {
			self.0.push(ToSqlSerializer.$fun()?);
			Ok(self.0)
		}
	};
}

/// Serializer into `PositionalParamSlice`
///
/// You shouldn't use it directly, but via the crate's `to_params()` function. Check the crate documentation for example.
pub struct PositionalSliceSerializer(pub PositionalParamSlice);

impl Default for PositionalSliceSerializer {
	fn default() -> Self {
		Self(PositionalParamSlice::from(Vec::new()))
	}
}

impl ser::Serializer for PositionalSliceSerializer {
	type Ok = PositionalParamSlice;
	type Error = Error;
	type SerializeSeq = Self;
	type SerializeTuple = Self;
	type SerializeTupleStruct = Self;
	type SerializeTupleVariant = Self;
	type SerializeMap = ser::Impossible<Self::Ok, Self::Error>;
	type SerializeStruct = ser::Impossible<Self::Ok, Self::Error>;
	type SerializeStructVariant = ser::Impossible<Self::Ok, Self::Error>;

	forward_tosql!(serialize_bool, bool);
	forward_tosql!(serialize_i8, i8);
	forward_tosql!(serialize_i16, i16);
	forward_tosql!(serialize_i32, i32);
	forward_tosql!(serialize_i64, i64);
	forward_tosql!(serialize_u8, u8);
	forward_tosql!(serialize_u16, u16);
	forward_tosql!(serialize_u32, u32);
	forward_tosql!(serialize_u64, u64);
	forward_tosql!(serialize_f32, f32);
	forward_tosql!(serialize_f64, f64);
	forward_tosql!(serialize_str, &str);
	forward_tosql!(serialize_char, char);
	forward_tosql!(serialize_bytes, &[u8]);
	forward_tosql!(serialize_none);
	forward_tosql!(serialize_unit);

	fn serialize_some<T: ?Sized + serde::Serialize>(self, value: &T) -> Result<Self::Ok> {
		value.serialize(self)
	}

	fn serialize_unit_struct(mut self, name: &'static str) -> Result<Self::Ok> {
		self.0.push(ToSqlSerializer.serialize_unit_struct(name)?);
		Ok(self.0)
	}

	fn serialize_unit_variant(mut self, name: &'static str, variant_index: u32, variant: &'static str) -> Result<Self::Ok> {
		self.0.push(ToSqlSerializer.serialize_unit_variant(name, variant_index, variant)?);
		Ok(self.0)
	}

	fn serialize_newtype_struct<T: ?Sized + serde::Serialize>(self, _name: &'static str, value: &T) -> Result<Self::Ok> {
		value.serialize(self)
	}

	fn serialize_newtype_variant<T: ?Sized + serde::Serialize>(self, _name: &'static str, _variant_index: u32, _variant: &'static str, value: &T) -> Result<Self::Ok> {
		value.serialize(self)
	}

	fn serialize_seq(mut self, len: Option<usize>) -> Result<Self::SerializeSeq> {
		if let Some(len) = len {
			self.0.reserve_exact(len);
		}
		Ok(self)
	}

	fn serialize_tuple(mut self, len: usize) -> Result<Self::SerializeTuple> {
		self.0.reserve_exact(len);
		Ok(self)
	}

	fn serialize_tuple_struct(mut self, _name: &'static str, len: usize) -> Result<Self::SerializeTupleStruct> {
		self.0.reserve_exact(len);
		Ok(self)
	}

	fn serialize_tuple_variant(mut self, _name: &'static str, _variant_index: u32, _variant: &'static str, len: usize) -> Result<Self::SerializeTupleVariant> {
		self.0.reserve_exact(len);
		Ok(self)
	}

	fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> { Err(Error::ser_unsupported("map")) }
	fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> { Err(Error::ser_unsupported("struct")) }
	fn serialize_struct_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize) -> Result<Self::SerializeStructVariant> { Err(Error::ser_unsupported("struct_variant")) }
}

impl ser::SerializeSeq for PositionalSliceSerializer {
	type Ok = PositionalParamSlice;
	type Error = Error;

	fn serialize_element<T: ?Sized + serde::Serialize>(&mut self, value: &T) -> Result<()> {
		self.0.push(value.serialize(ToSqlSerializer)?);
		Ok(())
	}

	fn end(self) -> Result<Self::Ok> {
		Ok(self.0)
	}
}

impl ser::SerializeTuple for PositionalSliceSerializer {
	type Ok = PositionalParamSlice;
	type Error = Error;

	fn serialize_element<T: ?Sized + serde::Serialize>(&mut self, value: &T) -> Result<()> {
		self.0.push(value.serialize(ToSqlSerializer)?);
		Ok(())
	}

	fn end(self) -> Result<Self::Ok> {
		Ok(self.0)
	}
}

impl ser::SerializeTupleStruct for PositionalSliceSerializer {
	type Ok = PositionalParamSlice;
	type Error = Error;

	fn serialize_field<T: ?Sized + serde::Serialize>(&mut self, value: &T) -> Result<()> {
		self.0.push(value.serialize(ToSqlSerializer)?);
		Ok(())
	}

	fn end(self) -> Result<Self::Ok> {
		Ok(self.0)
	}
}

impl ser::SerializeTupleVariant for PositionalSliceSerializer {
	type Ok = PositionalParamSlice;
	type Error = Error;

	fn serialize_field<T: ?Sized + serde::Serialize>(&mut self, value: &T) -> Result<()> {
		self.0.push(value.serialize(ToSqlSerializer)?);
		Ok(())
	}

	fn end(self) -> Result<Self::Ok> {
		Ok(self.0)
	}
}
