use super::Transformer;

macro_rules! impl_transformer_primitive {
    ($($args:ty;)*) => {
        $(
            impl Transformer for $args {
                type DataType = Self;

                fn transform(&self) -> Self::DataType {
                    *self
                }

                fn revert(value: &Self::DataType) -> Self {
                    *value
                }
            }
        )*
    }
}

impl_transformer_primitive! {
	u8;
	u16;
	u32;
	u64;
	usize;
	u128;
	i8;
	i16;
	i32;
	i64;
	isize;
	i128;
	bool;
}

impl Transformer for String {
	type DataType = Vec<u8>;

	fn transform(&self) -> Self::DataType {
		self.clone().into_bytes()
	}

	fn revert(value: &Self::DataType) -> Self {
		unsafe { Self::from_utf8_unchecked(value.clone()) }
	}
}

impl<Dt, T: Transformer<DataType = Dt>> Transformer for Option<T> {
	type DataType = Option<Dt>;

	fn transform(&self) -> Self::DataType {
		self.as_ref().map(Transformer::transform)
	}

	fn revert(value: &Self::DataType) -> Self {
		value.as_ref().map(T::revert)
	}
}
