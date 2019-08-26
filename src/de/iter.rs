use std::marker::PhantomData;

use rusqlite::{Row, Rows};
use serde::de::DeserializeOwned;

use crate::{Result, Error};

/// Iterator to automatically deserialize each row from owned `rusqlite::Rows` into `D: serde::Deserialize`
pub struct DeserRows<'stmt, D> {
	rows: Rows<'stmt>,
	columns: Option<Vec<String>>,
	d: PhantomData<*const D>,
}

impl<'stmt, D: DeserializeOwned> DeserRows<'stmt, D> {
	pub fn new(rows: Rows<'stmt>) -> Self {
		Self { columns: columns_from_rows(&rows), rows, d: PhantomData }
	}
}

impl<D: DeserializeOwned> Iterator for DeserRows<'_, D> {
	type Item = Result<D>;

	fn next(&mut self) -> Option<Self::Item> {
		deser_row(self.rows.next(), &self.columns)
	}
}

/// Iterator to automatically deserialize each row from borrowed `rusqlite::Rows` into `D: serde::Deserialize`
pub struct DeserRowsRef<'rows, 'stmt, D> {
	rows: &'rows mut Rows<'stmt>,
	columns: Option<Vec<String>>,
	d: PhantomData<*const D>,
}

impl<'rows, 'stmt, D: DeserializeOwned> DeserRowsRef<'rows, 'stmt, D> {
	pub fn new(rows: &'rows mut Rows<'stmt>) -> Self {
		Self { columns: columns_from_rows(&rows), rows, d: PhantomData }
	}
}

impl<D: DeserializeOwned> Iterator for DeserRowsRef<'_, '_, D> {
	type Item = Result<D>;

	fn next(&mut self) -> Option<Self::Item> {
		deser_row(self.rows.next(), &self.columns)
	}
}

#[inline]
fn deser_row<D: DeserializeOwned>(row: rusqlite::Result<Option<&Row>>, columns: &Option<Vec<String>>) -> Option<Result<D>> {
	if let Some(columns) = columns {
		match row {
			Ok(Some(row)) => Some(crate::from_row_with_columns(&row, columns)),
			Ok(None) => None,
			Err(e) => Some(Err(e.into())),
		}
	} else {
		Some(Err(Error::ColumnNamesNotAvalable))
	}
}

fn columns_from_rows(rows: &rusqlite::Rows) -> Option<Vec<String>> {
	rows.column_names()
		.map(|v| v.into_iter().map(|name| name.to_owned()).collect())
	/* // fixme: uncomment when https://github.com/jgallagher/rusqlite/pull/564 is merged and released
	rows.column_count()
		.and_then(|len| {
			let mut out = Vec::with_capacity(len);
			for i in 0..len {
				out.push(rows.column_name(i)?.to_owned())
			}
			Some(out)
		})
	*/
}
