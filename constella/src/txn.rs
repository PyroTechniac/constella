use crate::{
	mdb::{error::mdb_result, ffi},
	Env, Result,
};
use std::{marker::PhantomData, ops::Deref, ptr};

#[derive(Debug)]
pub struct RoTxn<'e, T = ()> {
	pub(crate) txn: *mut ffi::MDB_txn,
	pub(crate) env: &'e Env,
	_marker: PhantomData<T>,
}

impl<'e, T> RoTxn<'e, T> {
	pub(crate) fn new(env: &'e Env) -> Result<Self> {
		let mut txn = ptr::null_mut();

		unsafe {
			mdb_result(ffi::mdb_txn_begin(
				env.env_mut_ptr(),
				ptr::null_mut(),
				ffi::MDB_RDONLY,
				&mut txn,
			))?;
		};

		Ok(Self {
			txn,
			env,
			_marker: PhantomData,
		})
	}

	pub fn commit(mut self) -> Result<()> {
		let result = unsafe { mdb_result(ffi::mdb_txn_commit(self.txn)) };
		self.txn = ptr::null_mut();
		result.map_err(Into::into)
	}

	pub fn abort(mut self) -> Result<()> {
		let result = abort_txn(self.txn);
		self.txn = ptr::null_mut();
		result
	}
}

impl<T> Drop for RoTxn<'_, T> {
	fn drop(&mut self) {
		if !self.txn.is_null() {
			let val = abort_txn(self.txn);
			drop(val);
		}
	}
}

#[cfg(feature = "sync-read-txn")]
unsafe impl<T> Sync for RoTxn<'_, T> {}

#[cfg(feature = "lmdb")]
#[allow(clippy::unnecessary_wraps)]
fn abort_txn(txn: *mut ffi::MDB_txn) -> Result<()> {
	assert!(!txn.is_null());
	unsafe {
		ffi::mdb_txn_abort(txn);
	}
	Ok(())
}

#[cfg(feature = "mdbx")]
fn abort_txn(txn: &mut ffi::MDB_txn) -> Result<()> {
	assert!(!txn.is_null());

	let ret = unsafe { ffi::mdb_txn_abort(txn) };
	mdb_result(ret).map_err(Into::into)
}

#[derive(Debug)]
pub struct RwTxn<'e, 'p, T = ()> {
	pub(crate) txn: RoTxn<'e, T>,
	_parent: PhantomData<&'p mut ()>,
}

impl<'e, T> RwTxn<'e, 'e, T> {
	pub(crate) fn new(env: &'e Env) -> Result<Self> {
		let mut txn = ptr::null_mut();

		unsafe {
			mdb_result(ffi::mdb_txn_begin(
				env.env_mut_ptr(),
				ptr::null_mut(),
				0,
				&mut txn,
			))?;
		}

		Ok(Self {
			txn: RoTxn {
				txn,
				env,
				_marker: PhantomData,
			},
			_parent: PhantomData,
		})
	}

	pub(crate) fn nested<'p: 'e>(
		env: &'e Env,
		parent: &'p mut RwTxn<T>,
	) -> Result<RwTxn<'e, 'p, T>> {
		let mut txn = ptr::null_mut();
		let parent_ptr = parent.txn.txn;

		unsafe {
			mdb_result(ffi::mdb_txn_begin(
				env.env_mut_ptr(),
				parent_ptr,
				0,
				&mut txn,
			))?;
		}

		Ok(RwTxn {
			txn: RoTxn {
				txn,
				env,
				_marker: PhantomData,
			},
			_parent: PhantomData,
		})
	}

	pub fn commit(self) -> Result<()> {
		self.txn.commit()
	}

	pub fn abort(self) -> Result<()> {
		self.txn.abort()
	}
}

impl<'e, 'p, T> Deref for RwTxn<'e, 'p, T> {
	type Target = RoTxn<'e, T>;

	fn deref(&self) -> &Self::Target {
		&self.txn
	}
}
