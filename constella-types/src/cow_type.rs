use bytemuck::{bytes_of, bytes_of_mut, try_from_bytes, Pod, PodCastError};
use constella_traits::{BytesDecode, BytesEncode};
use std::{borrow::Cow, error::Error, marker::PhantomData};


/// Describes a type that must be [memory aligned] and
/// will be reallocated if it is not.
///
/// A [`Cow`] type is returned to represent this behavior.
///
/// If you need to store a type that doesn't depends on any
/// memory alignment it is recommended to use the [`UnalignedType`].
///
/// If you don't want to be bored with the [`Cow`] type you can
/// use the [`OwnedType`].
///
/// To store slices, you must look at the [`CowSlice`],
/// [`OwnedSlice`] or [`UnalignedSlice`] types.
///
/// [memory aligned]: std::mem::align_of()
/// [`Cow`]: std::borrow::Cow
/// [`UnalignedType`]: crate::UnalignedType
/// [`OwnedType`]: crate::OwnedType
/// [`UnalignedSlice`]: crate::UnalignedSlice
/// [`OwnedSlice`]: crate::OwnedSlice
/// [`CowSlice`]: crate::CowSlice
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CowType<T>(PhantomData<T>);

impl<T: Pod> BytesEncode for CowType<T> {
    type Item = T;

    fn bytes_encode(item: &Self::Item) -> Result<Cow<[u8]>, Box<dyn Error + Send + Sync>> {
        Ok(Cow::Borrowed(bytes_of(item)))
    }
}

impl<'a, T: Pod> BytesDecode<'a> for CowType<T> {
    type Item = Cow<'a, T>;

    fn bytes_decode(bytes: &'a [u8]) -> Result<Self::Item, Box<dyn Error + Send + Sync>> {
        match try_from_bytes(bytes) {
            Ok(item) => Ok(Cow::Borrowed(item)),
            Err(PodCastError::TargetAlignmentGreaterAndInputNotAligned) => {
                let mut item = T::zeroed();
                bytes_of_mut(&mut item).copy_from_slice(bytes);
                Ok(Cow::Owned(item))
            }
            Err(e) => Err(e.into())
        }
    }
}

unsafe impl<T> Send for CowType<T> {}
unsafe impl<T> Sync for CowType<T> {}