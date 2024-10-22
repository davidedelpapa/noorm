use std::ops::Deref;
use std::fmt::Write;
use serde::Deserialize;
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;
use sqlparser::ast::{Statement, ColumnOption};
use swc_ecma_ast::*;
use swc_common::{SourceMap, SyntaxContext, DUMMY_SP};
use swc_common::sync::Lrc;
use swc_ecma_codegen::{text_writer::JsWriter, Emitter};

/// AST type for Language::JavaScript(JavaScriptAst)
#[derive(Debug, Clone, Deserialize)]
pub struct JavaScriptAst(Vec<VarDecl>);

impl Default for JavaScriptAst {
    fn default() -> Self {
        let v: Vec<VarDecl> = Vec::new();
        JavaScriptAst(v)
    }
}

impl Deref for JavaScriptAst {
    type Target = Vec<VarDecl>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq for JavaScriptAst {
    fn eq(&self, other: &Self) -> bool {
        if self.0.len() != other.0.len() {
            return false;
        }

        // Compare the two ast by converting them to a `String`
        let s1 = javascript_ast_to_string(self);
        let s2 = javascript_ast_to_string(other);
        s1 == s2
    }
}

/// parse_create_table JavaScript implementation
pub fn javascript_parse_create_table(sql: &str) -> JavaScriptAst {
    let dialect = GenericDialect {}; // or MySqlDialect, PostgreSqlDialect, etc.
    let ast = Parser::parse_sql(&dialect, sql).expect("Failed to parse SQL");
    let mut js_objects = Vec::new();

    for stmt in ast {
        if let Statement::CreateTable(create_table) = stmt {
            let table_name = &create_table.name.0[0].value;
            //let struct_name = Ident::new(&capitalize_first_letter(table_name), proc_macro2::Span::call_site());

            // Prepare properties for the JavaScript object
            let mut props = Vec::new();
            
            for column in create_table.columns {
                let col_name = &column.name.value;
                let col_type = &column.data_type;
                let is_nullable = column.options.iter().any(|opt| opt.option == ColumnOption::Null);

                // Convert SQL type to JavaScript type
                let js_type = sql_to_type(col_type, is_nullable);

                // Generate a property for the JavaScript object
                let key = PropName::Ident(IdentName {
                    sym: col_name.to_string().into(),
                    span: DUMMY_SP,
                });

                props.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                    key,
                    value: js_type,
                }))));
            }

            // Create the full JavaScript object for the table
            let obj_lit = ObjectLit {
                span: DUMMY_SP,
                props,
            };

            // Create a variable declaration for the table object
            let var_decl = VarDecl {
                span: DUMMY_SP,
                decls: vec![VarDeclarator {
                    span: DUMMY_SP,
                    name: Pat::Ident(BindingIdent {
                        id: Ident {sym:table_name.to_string().into(),span:DUMMY_SP,optional:false, ctxt: SyntaxContext::default() },
                        type_ann: None,
                    }),
                    init: Some(Box::new(Expr::Object(obj_lit))),
                    definite: false,
                }],
                kind: VarDeclKind::Var,  // This uses 'var', you could also use 'let' or 'const'
                declare: false,
                ctxt: SyntaxContext::default(),
            };

            js_objects.push(var_decl);
        }
    } // End For statements
    JavaScriptAst(js_objects)
}

fn sql_to_type(sql_type: &sqlparser::ast::DataType, is_nullable: bool) -> Box<Expr> {
    // Example mapping logic - this will depend on your specific mappings
    let js_type = match sql_type {
        sqlparser::ast::DataType::Varchar(_) | sqlparser::ast::DataType::Text => "string",
        sqlparser::ast::DataType::Int(_) => "number",
        sqlparser::ast::DataType::Boolean => "boolean",
        _ => "any",  // Fallback to `any` for unsupported types
    };

    let type_expr = if is_nullable {
        format!("{} | null", js_type)
    } else {
        js_type.to_string()
    };

    // Return JavaScript expression for the type
    Box::new(Expr::Lit(Lit::Str(Str {
        value: type_expr.into(),
        span: DUMMY_SP,
        raw: None,
    })))
}


/// ast_to_string JavaScript implementation
pub fn javascript_ast_to_string(ast: &JavaScriptAst) -> String {
    let mut js_code = String::new();
    
    // Create a SourceMap to manage code generation
    let cm = Lrc::new(SourceMap::default());

    // Iterate over the variable declarations and generate code for each
    for var_decl in &ast.0 {
        let mut buf = vec![];
        {
            // Create a JsWriter to output JavaScript code into a buffer
            let writer = JsWriter::new(cm.clone(), "\n", &mut buf, None);
            
            // Create an emitter to generate the code
            let mut config = swc_ecma_codegen::Config::default();
            config.minify = false;

            let codegen_config = config;
            let mut emitter = Emitter {
                cfg: codegen_config,
                comments: None,
                cm: cm.clone(),
                wr: writer,
            };
        
    
            let boxed_decl = Box::new(var_decl.clone());
            let decl_var = Decl::Var(boxed_decl);
            emitter
                .emit_module_item(&ModuleItem::from(decl_var))
                .expect("Failed to emit JavaScript code");
        }
        let emitted_code = String::from_utf8_lossy(&buf);
        
        // Append a semicolon at the end of each declaration for valid JS
        writeln!(js_code, "{}", emitted_code).unwrap();

    }

    js_code
}
