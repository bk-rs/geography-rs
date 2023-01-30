//! [ISO 3166-2 - Wikipedia](https://en.wikipedia.org/wiki/ISO_3166-2)

//
#[macro_export]
macro_rules! country_subdivision_code {
    (
        $country_code_ty:ty, $country_code_val:expr;

        $( #[$meta:meta] )*
        $pub:vis enum $name:ident {
            $(
                $( #[$variant_meta:meta] )*
                $variant:ident,
            )+
        }
    ) => {
        $(#[$meta])*
        $pub enum $name {
            $(
                $( #[$variant_meta] )*
                $variant,
            )+
            Other($crate::alloc::boxed::Box<str>),
        }

        //
        impl $name {
            pub const COUNTRY_CODE: $country_code_ty = $country_code_val;

            pub const VARS: &'static [$name] = &[
                $(
                    $name::$variant,
                )+
            ];
        }

        //
        impl ::core::str::FromStr for $name {
            type Err = $crate::error::CountrySubdivisionCodeParseError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let country_code_s = s.chars().take_while(|x| x != &'-' && x != &'_')
                                                .collect::<$crate::alloc::string::String>();
                let country_code = country_code_s.parse::<$country_code_ty>()
                                                    .map_err(|_| $crate::error::CountrySubdivisionCodeParseError::CountryCodeInvalid(country_code_s.as_str().into()))?;

                if country_code != Self::COUNTRY_CODE {
                    return Err($crate::error::CountrySubdivisionCodeParseError::CountryCodeMismatch(country_code_s.into()));
                }

                let subdivision_code_s = if s.len() > country_code_s.len() + 1 {
                    &s[country_code_s.len() + 1..]
                } else {
                    return Err($crate::error::CountrySubdivisionCodeParseError::SubdivisionCodeMissing);
                };

                match subdivision_code_s {
                    $(
                        ::core::stringify!($variant) => Ok(Self::$variant),
                    )+
                    s if s.len() == 2 => Ok(Self::Other(s.into())),
                    s => Err($crate::error::CountrySubdivisionCodeParseError::SubdivisionCodeInvalid(s.into()))
                }
            }
        }

        //
        impl ::core::fmt::Display for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                match self {
                    $(
                        Self::$variant => ::core::write!(f, "{}-{}", $name::COUNTRY_CODE, ::core::stringify!($variant)),
                    )+
                    Self::Other(s) => ::core::write!(f, "{}-{}", $name::COUNTRY_CODE, s)
                }
            }
        }

        //
        impl ::core::default::Default for $name {
            fn default() -> Self {
                Self::Other(Default::default())
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
pub mod cn;
pub mod us;

pub use cn::CountrySubdivisionCode as CNSubdivisionCode;
pub use us::CountrySubdivisionCode as USSubdivisionCode;

//
//
//
use crate::iso3166_1::alpha_2::CountryCode;

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Hash))]
pub enum SubdivisionCode {
    CN(CNSubdivisionCode),
    US(USSubdivisionCode),
    Other(CountryCode, Option<::alloc::boxed::Box<str>>),
}

//
impl ::core::str::FromStr for SubdivisionCode {
    type Err = crate::error::CountrySubdivisionCodeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let country_code_s = s
            .chars()
            .take_while(|x| x != &'-' && x != &'_')
            .collect::<::alloc::string::String>();
        let country_code = country_code_s.parse::<CountryCode>().map_err(|_| {
            crate::error::CountrySubdivisionCodeParseError::CountryCodeInvalid(
                country_code_s.as_str().into(),
            )
        })?;

        match country_code {
            CountryCode::CN => {
                let subdivision = s.parse::<CNSubdivisionCode>()?;
                Ok(Self::CN(subdivision))
            }
            CountryCode::US => {
                let subdivision = s.parse::<USSubdivisionCode>()?;
                Ok(Self::US(subdivision))
            }
            country => {
                let subdivision_code_s = if s.len() > country_code_s.len() + 1 {
                    let subdivision_code_s = &s[country_code_s.len() + 1..];
                    Some(subdivision_code_s.into())
                } else {
                    None
                };

                Ok(Self::Other(country, subdivision_code_s))
            }
        }
    }
}

//
impl ::core::fmt::Display for SubdivisionCode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        match self {
            Self::CN(subdivision) => ::core::write!(f, "{subdivision}"),
            Self::US(subdivision) => ::core::write!(f, "{subdivision}"),
            Self::Other(country, Some(s)) => ::core::write!(f, "{country}-{s}"),
            Self::Other(country, None) => ::core::write!(f, "{country}-"),
        }
    }
}

//
impl_macros::impl_partial_eq_str_for_display! { str, SubdivisionCode }
impl_macros::impl_partial_eq_str_for_display! { &'a str, SubdivisionCode }
impl_macros::impl_partial_eq_str_for_display! { ::alloc::borrow::Cow<'a, str>, SubdivisionCode }
impl_macros::impl_partial_eq_str_for_display! { ::alloc::string::String, SubdivisionCode }

//
#[cfg(feature = "serde")]
impl<'de> ::serde::Deserialize<'de> for SubdivisionCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        use ::core::str::FromStr as _;

        let s = ::alloc::boxed::Box::<str>::deserialize(deserializer)?;
        Self::from_str(&s).map_err(::serde::de::Error::custom)
    }
}

//
#[cfg(feature = "serde")]
impl ::serde::Serialize for SubdivisionCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        use ::alloc::string::ToString as _;

        self.to_string().serialize(serializer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use alloc::string::ToString as _;

    #[test]
    fn test_subdivision_code() {
        //
        assert_eq!(
            SubdivisionCode::US(us::CountrySubdivisionCode::NY).to_string(),
            "US-NY"
        );
        assert_eq!(
            "US-NY".parse::<SubdivisionCode>().unwrap(),
            SubdivisionCode::US(us::CountrySubdivisionCode::NY)
        );

        //
        assert_eq!(
            SubdivisionCode::Other(CountryCode::ZW, Some("BU".into())).to_string(),
            "ZW-BU"
        );
        assert_eq!(
            "ZW-BU".parse::<SubdivisionCode>().unwrap(),
            SubdivisionCode::Other(CountryCode::ZW, Some("BU".into()))
        );

        //
        assert_eq!(
            SubdivisionCode::Other(CountryCode::AI, None).to_string(),
            "AI-"
        );
        assert_eq!(
            "AI-".parse::<SubdivisionCode>().unwrap(),
            SubdivisionCode::Other(CountryCode::AI, None)
        );
    }
}
