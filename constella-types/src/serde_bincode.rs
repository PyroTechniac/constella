use constella_traits::{BytesDecode, BytesEncode};
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, error::Error, marker::PhantomData};


/// Describes a type that is [`Serialize`]/[`Deserialize`] and uses `bincode` to do so.
///
/// It can borrow bytes from the original slice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bincode<T>(PhantomData<T>);

impl<T> BytesEncode for Bincode<T>
where
	T: Serialize,
{
	type Item = T;

	fn bytes_encode(item: &Self::Item) -> Result<Cow<[u8]>, Box<dyn Error + Send + Sync>> {
		bincode::serialize(item).map(Cow::Owned).map_err(Into::into)
	}
}

impl<'a, T: 'a> BytesDecode<'a> for Bincode<T> where T: Deserialize<'a> {
    type Item = T;

    fn bytes_decode(bytes: &'a [u8]) -> Result<Self::Item, Box<dyn Error + Send + Sync>> {
        bincode::deserialize(bytes).map_err(Into::into)
    }
}

unsafe impl<T> Send for Bincode<T> {}
unsafe impl<T> Sync for Bincode<T> {}