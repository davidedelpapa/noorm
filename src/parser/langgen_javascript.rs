use std::ops::Deref;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct JavaScriptAst(String);

impl Default for JavaScriptAst {
    fn default() -> Self {
        JavaScriptAst(String::new())
    }
}

impl Deref for JavaScriptAst {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// parse_create_table JavaScript implementation
pub fn javascript_parse_create_table(sql: &str) -> JavaScriptAst {
    JavaScriptAst(sql.to_string())
}

/// ast_to_string JavaScript implementation
pub fn javascript_ast_to_string(ast: &JavaScriptAst) -> String {
    ast.to_string()
}
