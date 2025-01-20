/// Helper macro to create a new enum type with a single field.
///
/// This macro is used to create a new enum type with a single field.
/// The enum type is derived with the `Clone`, `Copy`, `PartialEq`, `Eq`,
/// `PartialOrd`, `Ord`, and `Hash` traits. The enum type is also derived with
/// the `Debug` trait to provide a human-readable representation of the enum.
///
/// # Examples
///
/// ```rust,ignore
/// nutype_enum! {
///     pub enum AacPacketType(u8) {
///         SeqHdr = 0x0,
///         Raw = 0x1,
///     }
/// }
/// ```
macro_rules! nutype_enum {
    (
        $(#[$attr:meta])*
        $vis:vis enum $name:ident($type:ty) {
            $(
                $(#[$variant_attr:meta])*
                $variant:ident = $value:expr
            ),*$(,)?
        }
    ) => {
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        $(#[$attr])*
        #[repr(transparent)]
        $vis struct $name(pub $type);

        impl ::std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(
                        &$name::$variant => write!(f, "{}::{}", stringify!($name), stringify!($variant)),
                    )*
                    _ => write!(f, "{}({:?})", stringify!($name), self.0),
                }
            }
        }

        impl $name {
            $(
                $(#[$variant_attr])*
                #[allow(non_upper_case_globals)]
                pub const $variant: Self = Self($value);
            )*
        }

        impl From<$type> for $name {
            fn from(value: $type) -> Self {
                Self(value)
            }
        }

        impl From<$name> for $type {
            fn from(value: $name) -> Self {
                value.0
            }
        }
    };
}

pub(crate) use nutype_enum;
