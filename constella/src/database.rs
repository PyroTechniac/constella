use crate::{
	mdb::{error::mdb_result, ffi},
	types::DecodeIgnore,
	RoTxn,
};
use std::{
	borrow::Cow,
	marker::PhantomData,
	mem,
	ops::{Bound, RangeBounds},
	ptr,
};

#[derive(Debug)]
pub struct Database<K, V> {
	env_ident: usize,
	dbi: ffi::MDB_dbi,
	marker: PhantomData<(K, V)>,
}

impl<K, V> Database<K, V> {
	pub(crate) const fn new(env_ident: usize, dbi: ffi::MDB_dbi) -> Self {
		Self {
			env_ident,
			dbi,
			marker: PhantomData,
		}
	}

	#[cfg(feature = "mdbx")]
	pub fn sequence<T>(&self, txn: &RoTxn<T>) -> Result<u64> {
		assert_eq!(self.env_ident, txn.env.env_mut_ptr() as usize);

		let mut value = mem::MaybeUninit::uninit();

		let result = unsafe {
			mdb_result(ffi::mdb_dbi_sequence(
				txn.txn,
				self.dbi,
				value.as_mut_ptr(),
				0,
			))
		};

		match result {
			Ok(()) => unsafe { Ok(value.assume_init()) },
			Err(e) => Err(e.into()),
		}
	}
}
