mod cow_slice;
mod cow_type;
mod owned_slice;
mod owned_type;

pub mod integer;

#[cfg(feature = "bincode")]
mod serde_bincode;

pub use self::{cow_slice::CowSlice, cow_type::CowType, owned_slice::OwnedSlice, owned_type::OwnedType};
