use crate::{Error, Result};
use std::{error::Error as StdError, marker::PhantomData, result::Result as StdResult};

#[derive(Debug, Default)]
pub struct LazyDecode<C>(PhantomData<C>);

impl<'a, C: 'static> constella_traits::BytesDecode<'a> for LazyDecode<C> {
	type Item = Lazy<'a, C>;

	fn bytes_decode(bytes: &'a [u8]) -> StdResult<Self::Item, Box<dyn StdError + Send + Sync>> {
		Ok(Lazy {
			data: bytes,
			_marker: PhantomData,
		})
	}
}

#[derive(Debug, Clone, Copy)]
pub struct Lazy<'a, C> {
	data: &'a [u8],
	_marker: PhantomData<C>,
}

impl<'a, C: constella_traits::BytesDecode<'a>> Lazy<'a, C> {
	pub fn decode(&self) -> Result<C::Item> {
		C::bytes_decode(self.data).map_err(Error::Decoding)
	}
}
