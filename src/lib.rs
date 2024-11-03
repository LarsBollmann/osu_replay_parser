#![warn(rust_2018_idioms, missing_docs, unused)]

//! A library for parsing and analyzing osu! replay files.
#![doc = include_str!("../README.md")]

/// The parser module contains the functions and types for parsing osu! replay files.
pub mod parser;
/// The errors module contains the error types for the library.
pub mod errors;
/// The replay module contains the types for representing osu! replay files and handling of the compressed replay data.
pub mod replay;

pub use replay::{Replay, ReplayData};
pub use errors::ReplayDataError;