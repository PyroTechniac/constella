use std::{borrow::Cow, error::Error, marker::PhantomData};
use bytemuck::Pod;
use constella_traits::{BytesDecode, BytesEncode};
use crate::CowSlice;


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