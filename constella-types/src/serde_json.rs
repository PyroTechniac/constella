use std::{borrow::Cow, error::Error, marker::PhantomData};
use constella_traits::{BytesDecode, BytesEncode};
use serde::{Serialize, Deserialize};

/// Describes a type that is [`Serialize`]/[`Deserialize`] and uses `serde_json` to do so.
///
/// It can borrow bytes from the original slice.
#[doc(cfg(feature = "json"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Json<T>(PhantomData<T>);

impl<T> BytesEncode for Json<T> where T: Serialize {
    type Item =  T;

    fn bytes_encode(item: &Self::Item) -> Result<Cow<[u8]>, Box<dyn Error + Send + Sync>> {
        serde_json::to_vec(item).map(Cow::Owned).map_err(Into::into)
    }
}

impl<'a, T: 'a> BytesDecode<'a> for Json<T> where T: Deserialize<'a> {
    type Item = T;

    fn bytes_decode(bytes: &'a [u8]) -> Result<Self::Item, Box<dyn Error + Send + Sync>> {
        serde_json::from_slice(bytes).map_err(Into::into)
    }
}

unsafe impl<T> Send for Json<T> {}
unsafe impl<T> Sync for Json<T> {}