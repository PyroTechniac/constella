use std::{borrow::Cow, error::Error, marker::PhantomData};
use constella_traits::{BytesDecode, BytesEncode};
use bytemuck::Pod;
use crate::CowType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct OwnedType<T>(PhantomData<T>);

impl<T: Pod> BytesEncode for OwnedType<T> {
    type Item = T;

    fn bytes_encode(item: &Self::Item) -> Result<Cow<[u8]>, Box<dyn Error + Send + Sync>> {
        CowType::bytes_encode(item)
    }
}

impl<'a, T: Pod> BytesDecode<'a> for OwnedType<T> {
    type Item = T;

    fn bytes_decode(bytes: &'a [u8]) -> Result<Self::Item, Box<dyn Error + Send + Sync>> {
        CowType::<T>::bytes_decode(bytes).map(Cow::into_owned)
    }
}

unsafe impl<T> Send for OwnedType<T> {}
unsafe impl<T> Sync for OwnedType<T> {}