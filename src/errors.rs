use lzma_rs::error::Error as LzmaError;
use nom::error::{VerboseError, VerboseErrorKind};

/// Error type for parsing replay data
/// 
/// This error type is used for all errors that occur during parsing of replay data.

// TODO: Refactor error into parsing error and lzma error
pub enum ReplayDataError<'a> {
    /// Error parsing replay data
    /// This variant includes a trace of all the parsers that led to the error
    NomParsingError(VerboseError<&'a [u8]>),
    /// Expected value in replay data not found
    MissingValueError,
    /// Value in replay data is invalid
    InvalidValueError,
    /// Decompressed replay data is not valid UTF-8
    InvalidUtfError,
    /// Error decompressing replay data
    LzmaError(LzmaError)
}

impl<'a> From<ReplayDataError<'a>> for nom::Err<ReplayDataError<'a>> {
    fn from(e: ReplayDataError<'a>) -> Self {
        nom::Err::Error(e)
    }
}

pub(crate) fn from_context<I>(input: I, context: &'static str) -> VerboseError<I> {
    VerboseError {
        errors: vec![(input, VerboseErrorKind::Context(context))]
    }
}

// Take error trace and print human readable
fn convert_error(e: VerboseError<&[u8]>) -> String {
    let mut error = String::new();
    for (i, chunk) in e.errors.iter().rev().enumerate() {
        match chunk {
            (_, VerboseErrorKind::Context(s)) => {
                error.push_str(&format!("{}{}\n", "   ".repeat(i), s));
            }
            (_, VerboseErrorKind::Nom(e)) => {
                error.push_str(&format!("{}{}\n", "   ".repeat(i), e.description()))
            }
            _ => {}
        }
    }
    error
}

impl From<std::num::ParseIntError> for ReplayDataError<'_> {
    fn from(_: std::num::ParseIntError) -> Self {
        ReplayDataError::InvalidValueError
    }
}

impl From<std::num::ParseFloatError> for ReplayDataError<'_> {
    fn from(_: std::num::ParseFloatError) -> Self {
        ReplayDataError::InvalidValueError
    }
}

impl From<LzmaError> for ReplayDataError<'_> {
    fn from(lzma_error: lzma_rs::error::Error) -> Self {
        ReplayDataError::LzmaError(lzma_error)
    }
}

impl<'a> std::fmt::Debug for ReplayDataError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReplayDataError::NomParsingError(e) => write!(f, "\n{}", convert_error(e.clone())),
            ReplayDataError::MissingValueError => write!(f, "MissingValueError: Expected value in replay data not found"),
            ReplayDataError::InvalidValueError => write!(f, "InvalidValueError: Value in replay data is invalid"),
            ReplayDataError::InvalidUtfError => write!(f, "InvalidUtfError: Decompressed replay data is not valid UTF-8"),
            ReplayDataError::LzmaError(e) => write!(f, "LzmaError: Error decompressing replay data\n\n{}", e),
        }
    }
}

impl<'a> From<VerboseError<&'a [u8]>> for ReplayDataError<'a> {
    fn from(e: VerboseError<&'a [u8]>) -> Self {
        ReplayDataError::NomParsingError(e)
    }
}