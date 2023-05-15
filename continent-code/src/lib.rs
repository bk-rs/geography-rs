#![cfg_attr(not(feature = "std"), no_std)]

pub extern crate alloc;

//
#[macro_export]
macro_rules! continent_code {
    (
        $( #[$meta:meta] )*
        $pub:vis enum $name:ident {
            $(
                $( #[$variant_meta:meta] )*
                $variant:ident $( | $alias:ident )* => $variant_en_name:literal,
            )+
        }
    ) => {
        $(#[$meta])*
        $pub enum $name {
            $(
                $variant,
            )+
        }

        //
        impl $name {
            pub const VARS: &'static [$name] = &[
                $(
                    $( #[$variant_meta] )*
                    $name::$variant,
                )+
            ];

            pub fn en_name(&self) -> &'static str {
                match self {
                    $(
                        Self::$variant => $variant_en_name,
                    )+
                }
            }

            pub fn from_en_name(en_name: &str) -> Option<Self> {
                match en_name {
                    $(
                        $variant_en_name => Some(Self::$variant),
                    )+
                    _ => None,
                }
            }
        }

        //
        impl ::core::str::FromStr for $name {
            type Err = $crate::error::ParseError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $(
                        ::core::stringify!($variant) $( | ::core::stringify!($alias) )* | $variant_en_name => Ok(Self::$variant),
                    )+
                    s => Err($crate::error::ParseError::Invalid(s.into()))
                }
            }
        }

        //
        impl ::core::fmt::Display for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                match self {
                    $(
                        Self::$variant => ::core::write!(f, "{}", ::core::stringify!($variant)),
                    )+
                }
            }
        }

        //
        impl ::core::default::Default for $name {
            fn default() -> Self {
                Self::AS
            }
        }

        //
        impl ::core::cmp::PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                $crate::alloc::format!("{}", self) == $crate::alloc::format!("{}", other)
            }
        }

        impl ::core::cmp::Eq for $name {
        }

        //
        impl_macros::impl_partial_eq_str_for_display! { str, $name }
        impl_macros::impl_partial_eq_str_for_display! { &'a str, $name }
        impl_macros::impl_partial_eq_str_for_display! { $crate::alloc::borrow::Cow<'a, str>, $name }
        impl_macros::impl_partial_eq_str_for_display! { $crate::alloc::string::String, $name }

        //
        #[cfg(feature = "std")]
        impl ::std::hash::Hash for $name {
            fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
                $crate::alloc::format!("{}", self).hash(state);
            }
        }

        //
        #[cfg(feature = "serde")]
        impl<'de> ::serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                use ::core::str::FromStr as _;

                let s = $crate::alloc::boxed::Box::<str>::deserialize(deserializer)?;
                Self::from_str(&s).map_err(::serde::de::Error::custom)
            }
        }

        //
        #[cfg(feature = "serde")]
        impl ::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                use $crate::alloc::string::ToString as _;

                self.to_string().serialize(serializer)
            }
        }
    };
}

//
pub mod error;

//
pub mod seven;

pub use seven::ContinentCode;
