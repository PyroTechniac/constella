use std::{borrow::Cow, error::Error};

/// Trait marker for encoding bytes
pub trait BytesEncode {
	type Item: ?Sized;

	fn bytes_encode(item: &Self::Item) -> Result<Cow<[u8]>, Box<dyn Error + Send + Sync>>;
}

/// Trait marker for decoding bytes
pub trait BytesDecode<'a> {
	type Item: 'a;

	fn bytes_decode(bytes: &'a [u8]) -> Result<Self::Item, Box<dyn Error + Send + Sync>>;
}
