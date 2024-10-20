use std::ops::Deref;

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct PythonAst(String);

impl Default for PythonAst {
    fn default() -> Self {
        PythonAst(String::new())
    }
}

impl Deref for PythonAst {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


/// parse_create_table Python implementation
pub fn python_parse_create_table(sql: &str) -> PythonAst {
    PythonAst(sql.to_string())
}

/// ast_to_string Rust implementation
pub fn python_ast_to_string(ast: &PythonAst) -> String {
    ast.to_string()
}
