#![warn(missing_docs)]
//! # NoORM
//! A No-ORM Rust library to handle SQL in Rust without the use of an ORM

/// Parser module
pub mod parser;

/// The crate's prelude: it re-expors useful types.
pub mod prelude {
    pub use crate::parser::Dialect;
    pub use crate::parser::Parser;
    pub use crate::parser::ParserConfig;
}
