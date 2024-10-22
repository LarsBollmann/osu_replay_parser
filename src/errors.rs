use nom::error::ParseError;

#[derive(Debug)]
pub enum ReplayDataError {
    NomParsingError,
    ParseIntError,
    ParseFloatError,
    ParseStringError,
    MissingValueError,
    InvalidValueError,
    LzmaError,
}

impl ParseError<&[u8]> for ReplayDataError {
    fn from_error_kind(_input: &[u8], _: nom::error::ErrorKind) -> Self {
        ReplayDataError::NomParsingError
    }

    fn append(_input: &[u8], _kind: nom::error::ErrorKind, other: Self) -> Self {
        other
    }
}

impl From<ReplayDataError> for nom::Err<ReplayDataError> {
    fn from(e: ReplayDataError) -> Self {
        nom::Err::Error(e)
    }
}

impl From<std::num::ParseIntError> for ReplayDataError {
    fn from(_: std::num::ParseIntError) -> Self {
        ReplayDataError::ParseIntError
    }
}

impl From<std::num::ParseFloatError> for ReplayDataError {
    fn from(_: std::num::ParseFloatError) -> Self {
        ReplayDataError::ParseFloatError
    }
}

impl From<std::str::Utf8Error> for ReplayDataError {
    fn from(_: std::str::Utf8Error) -> Self {
        ReplayDataError::ParseStringError
    }
}
