use bytemuck::PodCastError;
use constella_traits::{BytesDecode, BytesEncode};
use std::{borrow::Cow, error::Error};

/// Describes the [`prim@unit`] type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Unit;

impl BytesEncode for Unit {
	type Item = ();

	fn bytes_encode(_: &Self::Item) -> Result<Cow<[u8]>, Box<dyn Error + Send + Sync>> {
		Ok(Cow::Borrowed(&[]))
	}
}

impl BytesDecode<'_> for Unit {
	type Item = ();

	fn bytes_decode(bytes: &[u8]) -> Result<Self::Item, Box<dyn Error + Send + Sync>> {
		if bytes.is_empty() {
			Ok(())
		} else {
			Err(PodCastError::SizeMismatch.into())
		}
	}
}
