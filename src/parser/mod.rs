mod parser;
mod errors;
mod langgen;
mod langgen_rust;
mod langgen_python;
mod langgen_javascript;

pub use parser::Dialect;
pub use parser::ParserConfig;
pub use parser::Parser;

pub use errors::ParserConfigError;

pub use langgen_rust::{RustAst, rust_ast_to_string, rust_parse_create_table};
pub use langgen_python::{PythonAst, python_parse_create_table, python_ast_to_string};
pub use langgen_javascript::{JavaScriptAst, javascript_parse_create_table, javascript_ast_to_string};

pub use langgen::Language;