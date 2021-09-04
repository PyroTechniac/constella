use libc::c_int;
use lmdb_sys as ffi;
use std::{error::Error as StdError, ffi::CStr, fmt, os::raw::c_char, str};

/// An LMDB error kind.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Error {
	/// key/data pair already exists.
	KeyExist,
	/// key/data pair not found (EOF).
	NotFound,
	/// Requested page not found - this usually indicates corruption.
	PageNotFound,
	/// Located page was wrong type.
	Corrupted,
	/// Update of meta page failed or environment had fatal error.
	Panic,
	/// Environment version mismatch.
	VersionMismatch,
	/// File is not a valid LMDB file.
	Invalid,
	/// Environment mapsize reached.
	MapFull,
	/// Environment maxdbs reached.
	DbsFull,
	/// Environment maxreaders reached.
	ReadersFull,
	/// Too many TLS keys in use - Windows only.
	TlsFull,
	/// Txn has too many dirty pages.
	TxnFull,
	/// Cursor stack too deep - internal error.
	CursorFull,
	/// Page has not enough space - internal error.
	PageFull,
	/// Database contents grew beyond environment mapsize.
	MapResized,
	/// Operation and DB incompatible, or DB type changed. This can mean:
	///   - The operation expects an MDB_DUPSORT / MDB_DUPFIXED database.
	///   - Opening a named DB when the unnamed DB has MDB_DUPSORT / MDB_INTEGERKEY.
	///   - Accessing a data record as a database, or vice versa.
	///   - The database was dropped and recreated with different flags.
	Incompatible,
	/// Invalid reuse of reader locktable slot.
	BadRslot,
	/// Transaction cannot recover - it must be aborted.
	BadTxn,
	/// Unsupported size of key/DB name/data, or wrong DUP_FIXED size.
	BadValSize,
	/// The specified DBI was changed unexpectedly.
	BadDbi,
	/// Other error.
	Other(c_int),
}

impl Error {
	#[must_use]
	pub fn not_found(self) -> bool {
		self == Self::NotFound
	}

	/// Converts a raw error code to an `Error`.
	#[must_use]
	pub const fn from_error_code(err_code: c_int) -> Self {
		match err_code {
			ffi::MDB_KEYEXIST => Self::KeyExist,
			ffi::MDB_NOTFOUND => Self::NotFound,
			ffi::MDB_PAGE_NOTFOUND => Self::PageNotFound,
			ffi::MDB_CORRUPTED => Self::Corrupted,
			ffi::MDB_PANIC => Self::Panic,
			ffi::MDB_VERSION_MISMATCH => Self::VersionMismatch,
			ffi::MDB_INVALID => Self::Invalid,
			ffi::MDB_MAP_FULL => Self::MapFull,
			ffi::MDB_DBS_FULL => Self::DbsFull,
			ffi::MDB_READERS_FULL => Self::ReadersFull,
			ffi::MDB_TLS_FULL => Self::TlsFull,
			ffi::MDB_TXN_FULL => Self::TxnFull,
			ffi::MDB_CURSOR_FULL => Self::CursorFull,
			ffi::MDB_PAGE_FULL => Self::PageFull,
			ffi::MDB_MAP_RESIZED => Self::MapResized,
			ffi::MDB_INCOMPATIBLE => Self::Incompatible,
			ffi::MDB_BAD_RSLOT => Self::BadRslot,
			ffi::MDB_BAD_TXN => Self::BadTxn,
			ffi::MDB_BAD_VALSIZE => Self::BadValSize,
			ffi::MDB_BAD_DBI => Self::BadDbi,
			other => Self::Other(other),
		}
	}

	#[must_use]
	pub const fn to_error_code(self) -> c_int {
        match self {
            Self::KeyExist => ffi::MDB_KEYEXIST,
            Self::NotFound => ffi::MDB_NOTFOUND,
            Self::PageNotFound => ffi::MDB_PAGE_NOTFOUND,
            Self::Corrupted => ffi::MDB_CORRUPTED,
            Self::Panic => ffi::MDB_PANIC,
            Self::VersionMismatch => ffi::MDB_VERSION_MISMATCH,
            Self::Invalid => ffi::MDB_INVALID,
            Self::MapFull => ffi::MDB_MAP_FULL,
            Self::DbsFull => ffi::MDB_DBS_FULL,
            Self::ReadersFull => ffi::MDB_READERS_FULL,
            Self::TlsFull => ffi::MDB_TLS_FULL,
            Self::TxnFull => ffi::MDB_TXN_FULL,
            Self::CursorFull => ffi::MDB_CURSOR_FULL,
            Self::PageFull => ffi::MDB_PAGE_FULL,
            Self::MapResized => ffi::MDB_MAP_RESIZED,
            Self::Incompatible => ffi::MDB_INCOMPATIBLE,
            Self::BadRslot => ffi::MDB_BAD_RSLOT,
            Self::BadTxn => ffi::MDB_BAD_TXN,
            Self::BadValSize => ffi::MDB_BAD_VALSIZE,
            Self::BadDbi => ffi::MDB_BAD_DBI,
            Self::Other(err_code) => err_code,
        }
    }
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let description = unsafe {
			let err = ffi::mdb_strerror(self.to_error_code()) as *const c_char;
			str::from_utf8_unchecked(CStr::from_ptr(err).to_bytes())
		};

		f.write_str(description)
	}
}

impl StdError for Error {}

pub const fn mdb_result(err_code: c_int) -> Result<(), Error> {
	if err_code == ffi::MDB_SUCCESS {
		Ok(())
	} else {
		Err(Error::from_error_code(err_code))
	}
}

#[cfg(test)]
mod test {
    use super::Error;

    #[test]
    fn description() {
        assert_eq!("Permission denied", Error::from_error_code(13).to_string());
        assert_eq!(
            "MDB_NOTFOUND: No matching key/data pair found",
            Error::NotFound.to_string()
        );
    }
}