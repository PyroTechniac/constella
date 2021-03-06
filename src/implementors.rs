use super::Transformer;
use structsy::PersistentEmbedded;
use twilight_model::id::{
	ApplicationId, AttachmentId, AuditLogEntryId, ChannelId, CommandId, EmojiId, GenericId,
	GuildId, IntegrationId, InteractionId, MessageId, RoleId, StageId, UserId, WebhookId,
};

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

macro_rules! impl_transformer_id {
	($($args:tt;)*) => {
		$(
			impl Transformer for $args {
				type DataType = u64;

				fn transform(&self) -> Self::DataType {
					self.0
				}

				fn revert(value: &Self::DataType) -> Self {
					Self(*value)
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
	u128;
	i8;
	i16;
	i32;
	i64;
	i128;
	bool;
	f32;
	f64;
}

impl_transformer_id! {
	ApplicationId;
	AttachmentId;
	AuditLogEntryId;
	ChannelId;
	CommandId;
	EmojiId;
	GenericId;
	GuildId;
	IntegrationId;
	InteractionId;
	MessageId;
	RoleId;
	StageId;
	UserId;
	WebhookId;
}

impl Transformer for String {
	type DataType = Self;

	fn transform(&self) -> Self::DataType {
		self.clone()
	}

	fn revert(value: &Self::DataType) -> Self {
		value.clone()
	}
}

#[cfg(target_pointer_width = "64")]
#[doc(cfg(target_pointer_width = "64"))]
impl Transformer for usize {
	type DataType = u64;

	fn transform(&self) -> Self::DataType {
		*self as u64
	}

	#[allow(clippy::cast_possible_truncation)]
	fn revert(value: &Self::DataType) -> Self {
		*value as Self
	}
}

#[cfg(target_pointer_width = "64")]
#[doc(cfg(target_pointer_width = "64"))]
impl Transformer for isize {
	type DataType = i64;

	fn transform(&self) -> Self::DataType {
		*self as i64
	}

	#[allow(clippy::cast_possible_truncation)]
	fn revert(value: &Self::DataType) -> Self {
		*value as Self
	}
}

impl<Dt: PersistentEmbedded, T: Transformer<DataType = Dt>> Transformer for Option<T> {
	type DataType = Option<Dt>;

	fn transform(&self) -> Self::DataType {
		self.as_ref().map(Transformer::transform)
	}

	fn revert(value: &Self::DataType) -> Self {
		value.as_ref().map(T::revert)
	}
}
