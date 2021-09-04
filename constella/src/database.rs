use crate::{
	mdb::{error::mdb_result, ffi},
	types::DecodeIgnore,
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
}
