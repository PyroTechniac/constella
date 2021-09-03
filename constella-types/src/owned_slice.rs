use std::{borrow::Cow, error::Error, marker::PhantomData};
use bytemuck::Pod;
use constella_traits::{BytesDecode, BytesEncode};
use crate::CowSlice;


/// Describes a [`Vec`] of types that are totally owned (doesn't
/// hold any reference to the original slice).
///
/// If you need to store a type that doesn't depends on any
/// [memory alignment] and that can be big it is recommended
/// to use the [`UnalignedSlice`].
///
/// The [`CowType`] is recommended for borrowed types (types that holds
/// references to the original slice).
///
/// [memory alignment]: std::mem::align_of()
/// [`UnalignedSlice`]: crate::UnalignedSlice
/// [`CowType`]: crate::CowType
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct OwnedSlice<'a, T>(PhantomData<&'a T>);

impl<'a, T: Pod> BytesEncode for OwnedSlice<'a, T> {
    type Item = &'a [T];

    fn bytes_encode(item: &Self::Item) -> Result<Cow<[u8]>, Box<dyn Error + Send + Sync>> {
        CowSlice::bytes_encode(item)
    }
}

impl<'a, T: Pod + 'a> BytesDecode<'a> for OwnedSlice<'_, T> {
    type Item = Vec<T>;

    fn bytes_decode(bytes: &'a [u8]) -> Result<Self::Item, Box<dyn Error + Send + Sync>> {
        CowSlice::bytes_decode(bytes).map(Cow::into_owned)
    }
}

unsafe impl<T> Send for OwnedSlice<'_, T> {}
unsafe impl<T> Sync for OwnedSlice<'_, T> {}