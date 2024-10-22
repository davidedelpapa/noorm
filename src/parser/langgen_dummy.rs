use std::ops::Deref;

use serde::Deserialize;

/// AST type for Language::Echo(EchoAst)
#[derive(Debug, Clone, Deserialize)]
pub struct DummyAst(String);

impl Default for DummyAst {
    fn default() -> Self {
        DummyAst(String::new())
    }
}

impl Deref for DummyAst {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq for DummyAst {
    fn eq(&self, other: &Self) -> bool {
        if self.0.len() != other.0.len() {
            return false;
        }

        // Compare the two ast by converting them to a `String`
        let s1 = dummy_ast_to_string(self);
        let s2 = dummy_ast_to_string(other);
        s1 == s2
    }
}

/// parse_create_table Python implementation
pub fn dummy_parse_create_table(sql: &str) -> DummyAst {
    DummyAst(sql.to_string())
}

/// ast_to_string Rust implementation
pub fn dummy_ast_to_string(ast: &DummyAst) -> String {
    ast.to_string()
}
