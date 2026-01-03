//
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    Invalid(::alloc::boxed::Box<str>),
}

impl core::fmt::Display for ParseError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl core::error::Error for ParseError {}

//
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CountrySubdivisionCodeParseError {
    CountryCodeInvalid(::alloc::boxed::Box<str>),
    CountryCodeMismatch(::alloc::boxed::Box<str>),
    SubdivisionCodeMissing,
    SubdivisionCodeInvalid(::alloc::boxed::Box<str>),
}

impl core::fmt::Display for CountrySubdivisionCodeParseError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl core::error::Error for CountrySubdivisionCodeParseError {}
