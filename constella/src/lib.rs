#![feature(once_cell)]
#![deny(clippy::all)]
#![warn(clippy::pedantic, clippy::nursery, clippy::suspicious)]
#![allow(
	clippy::module_name_repetitions,
	clippy::missing_safety_doc,
	clippy::missing_panics_doc,
	clippy::missing_errors_doc
)]

#[cfg(all(
	feature = "lmdb",
	feature = "mdbx",
	not(any(feature = "lmdb", feature = "mdbx"))
))]
compile_error!("Either lmdb or mdbx needs to be enabled, but not both");

mod cursor;
mod database;
mod env;
mod lazy_decode;
mod mdb;
mod txn;

pub use bytemuck;
pub use byteorder;
pub use constella_types as types;

pub use self::{
	cursor::{RoCursor, RwCursor},
	database::Database,
	env::{env_closing_event, Env, EnvClosingEvent, EnvOpenOptions},
	lazy_decode::{Lazy, LazyDecode},
	mdb::{
		error::Error as MdbError,
		ffi::{from_val, into_val},
		flags,
	},
	txn::{RoTxn, RwTxn},
};

use constella_traits as traits;
use std::{error::Error as StdError, fmt, io, result};

#[derive(Debug)]
pub enum Error {
	Io(io::Error),
	Mdb(MdbError),
	Encoding(Box<dyn StdError + Send + Sync>),
	Decoding(Box<dyn StdError + Send + Sync>),
	InvalidDatabaseTyping,
	DatabaseClosing,
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Io(error) => error.fmt(f),
			Self::Mdb(error) => error.fmt(f),
			Self::Encoding(e) => {
				f.write_str("error while encoding: ")?;
				e.fmt(f)
			}
			Self::Decoding(e) => {
				f.write_str("error while decoding: ")?;
				e.fmt(f)
			}
			Self::InvalidDatabaseTyping => {
				f.write_str("database was previously opened with different types")
			}
			Self::DatabaseClosing => {
				f.write_str("database is in a closing phase, you can't open it at the same time")
			}
		}
	}
}

impl StdError for Error {}

impl From<MdbError> for Error {
	fn from(error: MdbError) -> Self {
		match error {
			MdbError::Other(e) => Self::Io(io::Error::from_raw_os_error(e)),
			_ => Self::Mdb(error),
		}
	}
}

impl From<io::Error> for Error {
	fn from(error: io::Error) -> Self {
		Self::Io(error)
	}
}

pub type Result<T> = result::Result<T, Error>;
