use constella_traits::{BytesDecode, BytesEncode};

use crate::{
	mdb::{error::mdb_result, ffi},
	types::DecodeIgnore,
	Error, Result, RoCursor, RoTxn, RwTxn,
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

#[cfg(feature = "mdbx")]
#[doc(cfg(feature = "mdbx"))]
impl<K, V> Database<K, V> {
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

	pub fn increase_sequence<T>(&self, txn: &mut RwTxn<T>, increment: u64) -> Result<Option<u64>> {
		assert_eq!(self.env_ident, txn.txn.env.env_mut_ptr() as usize);

		use crate::mdb::error::Error;

		let mut value = mem::MaybeUninit::uninit();

		let result = unsafe {
			mdb_result(ffi::mdbx_dbi_sequence(
				txn.txn.txn,
				self.dbi,
				value.as_mut_ptr(),
				increment,
			))
		};

		match result {
			Ok(()) => unsafe { Ok(Some(value.assume_init())) },
			Err(Error::Other(c)) if c == i32::MAX => Ok(None),
			Err(e) => Err(e.into()),
		}
	}
}

impl<K, V> Database<K, V> {
	pub(crate) const fn new(env_ident: usize, dbi: ffi::MDB_dbi) -> Self {
		Self {
			env_ident,
			dbi,
			marker: PhantomData,
		}
	}

	pub fn get<'txn, T>(&self, txn: &'txn RoTxn<T>, key: &K::Item) -> Result<Option<V::Item>>
	where
		K: BytesEncode,
		V: BytesDecode<'txn>,
	{
		assert_eq!(self.env_ident, txn.env.env_mut_ptr() as usize);

		let key_bytes = K::bytes_encode(key).map_err(Error::Encoding)?;

		let mut key_val = unsafe { crate::into_val(&key_bytes) };
		let mut data_val = mem::MaybeUninit::uninit();

		let result = unsafe {
			mdb_result(ffi::mdb_get(
				txn.txn,
				self.dbi,
				&mut key_val,
				data_val.as_mut_ptr(),
			))
		};

		match result {
			Ok(()) => {
				let data = unsafe { crate::from_val(data_val.assume_init()) };
				let data = V::bytes_decode(data).map_err(Error::Decoding)?;
				Ok(Some(data))
			}
			Err(e) if e.not_found() => Ok(None),
			Err(e) => Err(e.into()),
		}
	}

	pub fn get_lower_than<'txn, T>(
		&self,
		txn: &'txn RoTxn<T>,
		key: &<K as BytesEncode>::Item,
	) -> Result<Option<(<K as BytesDecode<'txn>>::Item, V::Item)>>
	where
		K: BytesEncode + BytesDecode<'txn>,
		V: BytesDecode<'txn>,
	{
		assert_eq!(self.env_ident, txn.env.env_mut_ptr() as usize);

		let mut cursor = RoCursor::new(txn, self.dbi)?;
		let key_bytes = K::bytes_encode(key).map_err(Error::Encoding)?;
		cursor.move_on_key_greater_than_or_equal_to(&key_bytes)?;

		match cursor.move_on_prev() {
			Ok(Some((key, data))) => match (K::bytes_decode(key), V::bytes_decode(data)) {
				(Ok(key), Ok(data)) => Ok(Some((key, data))),
				(Err(e), _) | (_, Err(e)) => Err(Error::Decoding(e)),
			},
			Ok(None) => Ok(None),
			Err(e) => Err(e),
		}
	}
}
