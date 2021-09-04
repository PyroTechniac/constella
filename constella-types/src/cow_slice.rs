use bytemuck::{pod_collect_to_vec, try_cast_slice, Pod, PodCastError};
use constella_traits::{BytesDecode, BytesEncode};
use std::{borrow::Cow, error::Error, marker::PhantomData};

/// Describes a slice that must be [memory aligned] and
/// will be reallocated if it is not.
///
/// A [`Cow`] type is returned to represent this behavior.
///
/// If you need to store a slice that doesn't depends on any
/// memory alignment it is recommended to use the [`UnalignedSlice`].
///
/// if you don't want to be bored with the [`Cow`] type you can
/// use the [`OwnedSlice`].
///
/// [memory aligned]: std::mem::align_of()
/// [`Cow`]: std::borrow::Cow
/// [`UnalignedSlice`]: crate::UnalignedSlice
/// [`OwnedSlice`]: crate::OwnedSlice

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CowSlice<'a, T>(PhantomData<&'a T>);

impl<'a, T: Pod> BytesEncode for CowSlice<'a, T> {
	type Item = &'a [T];

	fn bytes_encode(item: &Self::Item) -> Result<Cow<[u8]>, Box<dyn Error + Send + Sync>> {
		try_cast_slice(item).map(Cow::Borrowed).map_err(Into::into)
	}
}

impl<'a, T: Pod> BytesDecode<'a> for CowSlice<'_, T> {
	type Item = Cow<'a, [T]>;

	fn bytes_decode(bytes: &'a [u8]) -> Result<Self::Item, Box<dyn Error + Send + Sync>> {
		match try_cast_slice(bytes) {
			Ok(items) => Ok(Cow::Borrowed(items)),
			Err(PodCastError::AlignmentMismatch) => Ok(Cow::Owned(pod_collect_to_vec(bytes))),
			Err(e) => Err(e.into()),
		}
	}
}

unsafe impl<T> Send for CowSlice<'_, T> {}
unsafe impl<T> Sync for CowSlice<'_, T> {}
