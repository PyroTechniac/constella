use crate::{
	mdb::{error::mdb_result, ffi},
	Result, RoTxn, RwTxn,
};
use std::{
	marker::PhantomData,
	mem,
	ops::{Deref, DerefMut},
	ptr,
};

type MoveResult<'txn> = Result<Option<(&'txn [u8], &'txn [u8])>>;

#[derive(Debug)]
pub struct RoCursor<'txn> {
	cursor: *mut ffi::MDB_cursor,
	_marker: PhantomData<&'txn ()>,
}

impl<'txn> RoCursor<'txn> {
	pub(crate) fn new<T>(txn: &'txn RoTxn<T>, dbi: ffi::MDB_dbi) -> Result<Self> {
		let mut cursor = ptr::null_mut();

		unsafe {
			mdb_result(ffi::mdb_cursor_open(txn.txn, dbi, &mut cursor))?;
		}

		Ok(Self {
			cursor,
			_marker: PhantomData,
		})
	}

	pub fn current(&mut self) -> MoveResult {
		self.raw_move(ffi::cursor_op::MDB_GET_CURRENT)
	}

	pub fn move_on_first(&mut self) -> MoveResult {
		self.raw_move(ffi::cursor_op::MDB_FIRST)
	}

	pub fn move_on_last(&mut self) -> MoveResult {
		self.raw_move(ffi::cursor_op::MDB_LAST)
	}

	pub fn move_on_prev(&mut self) -> MoveResult {
		self.raw_move(ffi::cursor_op::MDB_PREV)
	}

	pub fn move_on_next(&mut self) -> MoveResult {
		self.raw_move(ffi::cursor_op::MDB_NEXT)
	}

	fn raw_move(&mut self, cursor: ffi::MDB_cursor_op) -> MoveResult {
		let mut key_val = mem::MaybeUninit::uninit();
		let mut data_val = mem::MaybeUninit::uninit();

		let result = unsafe {
			mdb_result(ffi::mdb_cursor_get(
				self.cursor,
				key_val.as_mut_ptr(),
				data_val.as_mut_ptr(),
				cursor,
			))
		};

		match result {
			Ok(()) => {
				let (key, data) = unsafe {
					(
						crate::from_val(key_val.assume_init()),
						crate::from_val(data_val.assume_init()),
					)
				};

				Ok(Some((key, data)))
			}
			Err(e) if e.not_found() => Ok(None),
			Err(e) => Err(e.into()),
		}
	}
}

impl Drop for RoCursor<'_> {
	fn drop(&mut self) {
		unsafe { ffi::mdb_cursor_close(self.cursor) }
	}
}

#[derive(Debug)]
pub struct RwCursor<'txn> {
	cursor: RoCursor<'txn>,
}

impl<'txn> RwCursor<'txn> {
	pub(crate) fn new<T>(txn: &'txn RwTxn<T>, dbi: ffi::MDB_dbi) -> Result<Self> {
		Ok(Self {
			cursor: RoCursor::new(txn, dbi)?,
		})
	}

	pub fn del_current(&mut self) -> Result<bool> {
		let result = unsafe { mdb_result(ffi::mdb_cursor_del(self.cursor.cursor, 0)) };

		match result {
			Ok(()) => Ok(true),
			Err(e) if e.not_found() => Ok(false),
			Err(e) => Err(e.into()),
		}
	}

	pub fn put_current(&mut self, key: &[u8], data: &[u8]) -> Result<bool> {
		let (mut key_val, mut data_val) =
			unsafe { (crate::into_val(key), crate::into_val(data)) };

		let result = unsafe {
			mdb_result(ffi::mdb_cursor_put(
				self.cursor.cursor,
				&mut key_val,
				&mut data_val,
				ffi::MDB_CURRENT,
			))
		};

		match result {
			Ok(()) => Ok(true),
			Err(e) if e.not_found() => Ok(false),
			Err(e) => Err(e.into()),
		}
	}

    pub fn append(&mut self, key: &[u8], data: &[u8]) -> Result<()> {
        let (mut key_val, mut data_val) = unsafe {
            (crate::into_val(key), crate::into_val(data))
        };
    }
}
