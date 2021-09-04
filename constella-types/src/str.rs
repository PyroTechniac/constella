use bytemuck::try_cast_slice;
use constella_traits::{BytesDecode, BytesEncode};
use std::{borrow::Cow, error::Error, marker::PhantomData, str};

/// Describes an [`prim@str`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Str<'a> {
	_phantom: PhantomData<&'a ()>,
}

impl<'a> BytesEncode for Str<'a> {
	type Item = &'a str;

	fn bytes_encode(item: &Self::Item) -> Result<Cow<[u8]>, Box<dyn Error + Send + Sync>> {
		try_cast_slice(item.as_bytes())
			.map(Cow::Borrowed)
			.map_err(Into::into)
	}
}

impl<'a> BytesDecode<'a> for Str<'_> {
	type Item = &'a str;

	fn bytes_decode(bytes: &'a [u8]) -> Result<Self::Item, Box<dyn Error + Send + Sync>> {
		str::from_utf8(bytes).map_err(Into::into)
	}
}
