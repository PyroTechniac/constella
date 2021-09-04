#![feature(doc_cfg)]

//! Types that can be used to serialize and deserialize types inside databases.
//!
//! How to choose the right type to store things in this database?
//! For specific types you can choose:
//!   - [`Str`] to store [`str`](primitive@str)s
//!   - [`Unit`] to store `()` types
//!   - [`Bincode`] or [`Json`] to store [`serde::Serialize`]/[`serde::Deserialize`] types
//!
//! But if you want to store big types that can be efficiently deserialized then
//! here is a little table to help you in your quest:
//!
//! | Available types    | Encoding type | Decoding type | allocations                                              |
//! |--------------------|:-------------:|:-------------:|----------------------------------------------------------|
//! | [`CowSlice`]       | `&[T]`        | `Cow<[T]>`    | will allocate if memory is miss-aligned                  |
//! | [`CowType`]        | `&T`          | `Cow<T>`      | will allocate if memory is miss-aligned                  |
//! | [`OwnedSlice`]     | `&[T]`        | `Vec<T>`      | will _always_ allocate                                   |
//! | [`OwnedType`]      | `&T`          | `T`           | will _always_ allocate                                   |
//! | [`UnalignedSlice`] | `&[T]`        | `&[T]`        | will _never_ allocate because alignment is always valid  |
//! | [`UnalignedType`]  | `&T`          | `&T`          | will _never_ allocate because alignment is always valid  |
//!
//! [`Serialize`]: serde::Serialize
//! [`Deserialize`]: serde::Deserialize

mod cow_slice;
mod cow_type;
mod owned_slice;
mod owned_type;
mod str;
mod unaligned_slice;
mod unaligned_type;
mod unit;

pub mod integer;

#[cfg(feature = "bincode")]
mod serde_bincode;

#[cfg(feature = "json")]
mod serde_json;

pub use self::{
	cow_slice::CowSlice, cow_type::CowType, owned_slice::OwnedSlice, owned_type::OwnedType,
	str::Str, unaligned_slice::UnalignedSlice, unaligned_type::UnalignedType, unit::Unit,
};

#[cfg(feature = "bincode")]
pub use self::serde_bincode::Bincode;

#[cfg(feature = "json")]
pub use self::serde_json::Json;

use std::error::Error;

/// Describes a slice of bytes `[u8]` that is totally
/// borrowed and doesn't depends on any [memory alignment].
///
/// [memory alignment]: std::mem::align_of()
pub type ByteSlice<'a> = UnalignedSlice<'a, u8>;

/// A convenient struct made to ignore the type when decoding it.
///
/// It is appropriate to be used to count keys for example
/// or to ensure that an entry exist for example.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DecodeIgnore;

impl constella_traits::BytesDecode<'_> for DecodeIgnore {
	type Item = ();

	fn bytes_decode(_: &[u8]) -> Result<Self::Item, Box<dyn Error + Send + Sync>> {
		Ok(())
	}
}