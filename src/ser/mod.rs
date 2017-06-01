macro_rules! ser_unimpl {
	($fn:ident, $type:ty) => {
		fn $fn(self, _v: $type) -> Result<Self::Ok> {
			Err(Error::ser_unsupported(stringify!($type)))
		}
	}
}

mod blob;
mod named;
mod positional;
mod slice;
mod tosql;

pub use super::{Error, ErrorKind, Result};
pub use self::named::NamedSliceSerializer;
pub use self::positional::PositionalSliceSerializer;
pub use self::slice::{PositionalParamSlice, NamedParamSlice};
