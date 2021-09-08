#![deny(clippy::all)]
#![warn(clippy::pedantic, clippy::nursery, clippy::suspicious)]

use std::{
	cmp::Ordering,
	io::{Read, Write},
	marker::PhantomData,
};
use structsy::{PersistentEmbedded, SRes};

mod implementors;

pub trait Transformer {
	type DataType;

	fn transform(&self) -> Self::DataType;

	fn revert(value: &Self::DataType) -> Self;
}

#[derive(Debug)]
pub struct DataHolder<V, T> {
	inner: V,
	_marker: PhantomData<T>,
}

impl<V, T> DataHolder<V, T> {
	pub const fn new(data: V) -> Self {
		Self {
			inner: data,
			_marker: PhantomData,
		}
	}
}

impl<V, T> DataHolder<V, T>
where
	T: Transformer<DataType = V>,
{
	pub fn from_value(value: &T) -> Self {
		Self {
			inner: value.transform(),
			_marker: PhantomData,
		}
	}

	pub fn into_value(&self) -> T {
		T::revert(&self.inner)
	}
}

impl<V, T> From<T> for DataHolder<V, T>
where
	T: Transformer<DataType = V>,
{
	fn from(value: T) -> Self {
		let inner = value.transform();

		Self {
			inner,
			_marker: PhantomData,
		}
	}
}

impl<V, T> Default for DataHolder<V, T>
where
	T: Transformer<DataType = V> + Default,
{
	fn default() -> Self {
		let inner = T::default().transform();

		Self {
			inner,
			_marker: PhantomData,
		}
	}
}

impl<V: Clone, T> Clone for DataHolder<V, T> {
	fn clone(&self) -> Self {
		Self {
			inner: self.inner.clone(),
			_marker: PhantomData,
		}
	}
}

impl<V: Copy, T> Copy for DataHolder<V, T> {}

impl<V: PartialEq, T> PartialEq for DataHolder<V, T> {
	fn eq(&self, other: &Self) -> bool {
		self.inner.eq(&other.inner)
	}
}

impl<V: Eq, T> Eq for DataHolder<V, T> {}

impl<V: PartialOrd, T> PartialOrd for DataHolder<V, T> {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		self.inner.partial_cmp(&other.inner)
	}
}

impl<V: Ord, T> Ord for DataHolder<V, T> {
	fn cmp(&self, other: &Self) -> Ordering {
		self.inner.cmp(&other.inner)
	}
}

unsafe impl<V: Send, T> Send for DataHolder<V, T> {}
unsafe impl<V: Sync, T> Sync for DataHolder<V, T> {}
