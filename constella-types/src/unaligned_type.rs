use std::{borrow::Cow, error::Error, marker::PhantomData};
use bytemuck::{Pod, bytes_of, try_from_bytes};
use constella_traits::{BytesDecode, BytesEncode};


/// Describes a type that is totally borrowed and doesn't
/// depends on any [memory alignment].
///
/// If you need to store a type that does depend on memory alignment
/// and that can be big it is recommended to use the [`CowType`].
///
/// To store slices, you must look at the [`CowSlice`],
/// [`OwnedSlice`] or [`UnalignedSlice`] types.
///
/// [memory alignment]: std::mem::align_of()
/// [`CowType`]: crate::CowType
/// [`UnalignedSlice`]: crate::UnalignedSlice
/// [`OwnedSlice`]: crate::OwnedSlice
/// [`CowSlice`]: crate::CowSlice
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct UnalignedType<T>(PhantomData<T>);

impl<T: Pod> BytesEncode for UnalignedType<T> {
    type Item = T;

    fn bytes_encode(item: &Self::Item) -> Result<Cow<[u8]>, Box<dyn Error + Send + Sync>> {
        Ok(Cow::Borrowed(bytes_of(item)))
    }
}

impl<'a, T: Pod> BytesDecode<'a> for UnalignedType<T> {
    type Item = &'a T;

    fn bytes_decode(bytes: &'a [u8]) -> Result<Self::Item, Box<dyn Error + Send + Sync>> {
        try_from_bytes(bytes).map_err(Into::into)
    }
}

unsafe impl<T> Send for UnalignedType<T> {}
unsafe impl<T> Sync for UnalignedType<T> {}