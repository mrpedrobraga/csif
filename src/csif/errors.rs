use std::{array::TryFromSliceError, str::Utf8Error};

#[derive(Debug, Clone)]
pub enum ParsingError {
    Field(Box<ParsingError>),
    OutOfBounds(std::ops::Range<usize>),
    FromSliceError(TryFromSliceError),
    Utf8Error(Utf8Error)
}