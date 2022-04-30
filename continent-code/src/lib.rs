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
                $variant:ident $( | $alias:ident )*,
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
        }

        //
        impl ::core::str::FromStr for $name {
            type Err = $crate::error::ParseError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $(
                        ::core::stringify!($variant) $( | ::core::stringify!($alias) )* => Ok(Self::$variant),
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
continent_code! {
    #[derive(Debug, Clone)]
    pub enum ContinentCode {
        AS,
        AF,
        NA,
        SA,
        AN,
        EU,
        OC | AU,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use alloc::string::ToString as _;

    use csv::ReaderBuilder;

    #[test]
    fn test_continent_code() {
        // Wikipedia
        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .flexible(true)
            .from_reader(include_str!("../tests/Continent/Area_and_population.csv").as_bytes());

        let mut n = 0;
        for record in rdr.records().skip(2) {
            let record = record.unwrap();
            let name = &record[0];
            let code = match name {
                "Asia" => "AS",
                "Africa" => "AF",
                "North America" => "NA",
                "South America" => "SA",
                "Antarctica" => "AN",
                "Europe" => "EU",
                "Oceania" => "OC",
                s => panic!("{}", s),
            };
            assert_eq!(code.parse::<ContinentCode>().unwrap().to_string(), code);
            n += 1;
        }

        assert_eq!("AU".parse::<ContinentCode>().unwrap().to_string(), "OC");

        assert_eq!(ContinentCode::VARS.len(), n);

        // PartialEq
        assert_eq!(ContinentCode::AS, ContinentCode::AS);
        assert_eq!(ContinentCode::AS, "AS");

        #[cfg(feature = "serde")]
        {
            #[derive(serde::Serialize, serde::Deserialize)]
            struct Foo {
                code: ContinentCode,
            }

            assert_eq!(
                serde_json::from_str::<Foo>(r#"{"code":"AS"}"#)
                    .unwrap()
                    .code,
                ContinentCode::AS
            );
            assert_eq!(
                serde_json::to_string(&Foo {
                    code: ContinentCode::AS
                })
                .unwrap(),
                r#"{"code":"AS"}"#
            );
        }
    }
}
