use serde::Deserialize;

use super::{
    RustAst, rust_ast_to_string, rust_parse_create_table,
    JavaScriptAst, javascript_parse_create_table, javascript_ast_to_string,
    PythonAst, python_parse_create_table, python_ast_to_string, 
};


/// Language selection enum
#[derive(Debug, Clone, Deserialize)]
pub enum Language {
    Rust(RustAst),
    Python(PythonAst),
    JavaScript(JavaScriptAst),
}

impl Default for Language {
    fn default() -> Self {
        let rust_ast = RustAst::default();
        Language::Rust(rust_ast)
    }
}

impl Language {
    pub fn new(language_name: &str) -> Option<Self> {
        match language_name.to_lowercase().as_str() {
            "rust" => {
                let ast = RustAst::default();
                Some(Language::Rust(ast))
            }
            "python" => {
                let ast = PythonAst::default();
                Some(Language::Python(ast))
            }
            "javascript" => {
                let ast = JavaScriptAst::default();
                Some(Language::JavaScript(ast))
            }
            _ => None, // Handle unsupported languages
        }
    }

    /// Implement conversion to string for the language
    pub fn parse_create_table(&mut self, sql: &str) {
        match self {
            Language::Rust(ast) => *ast = rust_parse_create_table(sql),
            Language::Python(ast) => *ast = python_parse_create_table(sql),
            Language::JavaScript(ast) => *ast = javascript_parse_create_table(sql),
            _ => unreachable!(),
        }
    }

    /// Implement conversion to string for the language
    pub fn to_string(&self) -> String {
        match self {
            Language::Rust(fn_ast) => rust_ast_to_string(fn_ast),
            Language::Python(fn_ast) => python_ast_to_string(fn_ast),
            Language::JavaScript(fn_ast) => javascript_ast_to_string(fn_ast),
            _ => unreachable!(),
        }
    }
}
