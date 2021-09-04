use bytemuck::{try_cast_slice, Pod};
use constella_traits::{BytesDecode, BytesEncode};
use std::{borrow::Cow, error::Error, marker::PhantomData};

/// Describes a slice that is totally borrowed and doesn't
/// depends on any [memory alignment].
///
/// If you need to store a slice that does depend on memory alignment
/// and that can be big it is recommended to use the [`CowSlice`].
///
/// [memory alignment]: std::mem::align_of()
/// [`CowSlice`]: crate::CowSlice
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct UnalignedSlice<'a, T>(PhantomData<&'a T>);

impl<'a, T: Pod> BytesEncode for UnalignedSlice<'a, T> {
	type Item = &'a [T];

	fn bytes_encode(item: &Self::Item) -> Result<Cow<[u8]>, Box<dyn Error + Send + Sync>> {
		try_cast_slice(item).map(Cow::Borrowed).map_err(Into::into)
	}
}

impl<'a, T: Pod> BytesDecode<'a> for UnalignedSlice<'_, T> {
	type Item = &'a [T];

	fn bytes_decode(bytes: &'a [u8]) -> Result<Self::Item, Box<dyn Error + Send + Sync>> {
		try_cast_slice(bytes).map_err(Into::into)
	}
}

unsafe impl<T> Send for UnalignedSlice<'_, T> {}
unsafe impl<T> Sync for UnalignedSlice<'_, T> {}
