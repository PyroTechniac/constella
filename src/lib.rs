#![feature(doc_cfg)]
#![deny(clippy::all)]
#![warn(clippy::pedantic, clippy::nursery, clippy::suspicious)]

use std::{
	cmp::Ordering,
	fmt::{Debug, Formatter, Result as FmtResult},
	io::{Read, Write},
	marker::PhantomData,
};
use structsy::{Persistent, PersistentEmbedded, Ref, SRes, internal::IndexableValue};

mod implementors;

pub trait Transformer {
	type DataType: PersistentEmbedded;

	fn transform(&self) -> Self::DataType;

	fn revert(value: &Self::DataType) -> Self;
}

pub struct DataHolder<V, T> {
	inner: V,
	_marker: PhantomData<T>,
}

pub type DataTransformer<V> = DataHolder<<V as Transformer>::DataType, V>;

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

impl<V, T> IndexableValue for DataHolder<V, T>
where
	V: IndexableValue,
	T: Transformer<DataType = V>,
{
	fn puts<P: Persistent>(&self, tx: &mut dyn structsy::Sytx, name: &str, id: &Ref<P>) -> SRes<()> {
		self.inner.puts(tx, name, id)
	}

	fn removes<P: Persistent>(&self, tx: &mut dyn structsy::Sytx, name: &str, id: &Ref<P>) -> SRes<()> {
		self.inner.removes(tx, name, id)
	}
}

impl<V, T> PersistentEmbedded for DataHolder<V, T>
where
	V: PersistentEmbedded,
	T: Transformer<DataType = V>,
{
	fn read(read: &mut dyn Read) -> SRes<Self>
	where
		Self: Sized,
	{
		let inner = V::read(read)?;

		Ok(Self {
			inner,
			_marker: PhantomData,
		})
	}

	fn write(&self, write: &mut dyn Write) -> SRes<()> {
		self.inner.write(write)
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

impl<V, T> Debug for DataHolder<V, T>
where
	V: Debug,
{
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.debug_struct("DataHolder")
			.field("inner", &self.inner)
			.field("_marker", &"_")
			.finish()
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

impl<V: PartialEq, T> PartialEq<V> for DataHolder<V, T> {
	fn eq(&self, other: &V) -> bool {
		self.inner.eq(other)
	}
}

impl<V: PartialOrd, T> PartialOrd<V> for DataHolder<V, T> {
	fn partial_cmp(&self, other: &V) -> Option<Ordering> {
		self.inner.partial_cmp(other)
	}
}

unsafe impl<V: Send, T> Send for DataHolder<V, T> {}
unsafe impl<V: Sync, T> Sync for DataHolder<V, T> {}

#[cfg(test)]
mod tests {
	use super::{DataHolder, Transformer};

	#[derive(Debug, Default, PartialEq)]
	struct Id(pub u64);

	impl Transformer for Id {
		type DataType = u64;

		fn transform(&self) -> Self::DataType {
			self.0
		}

		fn revert(value: &Self::DataType) -> Self {
			Self(*value)
		}
	}

	#[test]
	fn persistent_embed() {
		let value = Id::default();

		let wrapper = DataHolder::from(value);

		assert_eq!(wrapper, 0);

		let value_reverted = wrapper.into_value();

		assert_eq!(value_reverted, Id(0));
	}
}
