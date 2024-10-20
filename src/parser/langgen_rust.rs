use serde::de::{self, Deserializer, Visitor, SeqAccess};
use serde::Deserialize;
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;
use sqlparser::ast::{Statement, DataType, ColumnOption};
use syn::token::Pub;
use syn::{parse_str, Field, Ident, ItemStruct, Type, Visibility};
use proc_macro2::{self, Span};
use quote::ToTokens;
use std::ops::Deref;
use std::fmt;

#[derive(Clone)]
pub struct RustAst(Vec<ItemStruct>);

impl fmt::Debug for RustAst {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RustAst")
            .field("inner", &"Vec<ItemStruct>")  // Placeholder for field2
            .finish()
    }
}

impl Default for RustAst {
    fn default() -> Self {

        let v = Vec::new();
        let mut ast = RustAst(v);
        ast.0.push(ItemStruct {
            attrs: Vec::new(),
            vis: Visibility::Public(Pub::default()),
            struct_token: Default::default(),
            ident: Ident::new("emptyIdent", Span::call_site()),
            generics: Default::default(),
            fields: syn::Fields::Named(syn::FieldsNamed {
                brace_token: Default::default(),
                named: Default::default(),
            }),
            semi_token: None,
        });
        ast
    }
}
impl Deref for RustAst {
    type Target = Vec<ItemStruct>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'de> Deserialize<'de> for RustAst {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(RustAstVisitor)
    }
}

// Define the visitor for RustAst
struct RustAstVisitor;

impl<'de> Visitor<'de> for RustAstVisitor {
    type Value = RustAst;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a sequence of Rust struct definitions as strings")
    }

    // Visit the sequence and parse each item as a Rust struct using syn::parse_str
    fn visit_seq<A>(self, mut seq: A) -> Result<RustAst, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut items = Vec::new();

        while let Some(value) = seq.next_element::<String>()? {
            match parse_str::<ItemStruct>(&value) {
                Ok(item_struct) => items.push(item_struct),
                Err(_) => return Err(de::Error::custom("Failed to parse Rust struct")),
            }
        }

        Ok(RustAst(items))
    }
}



/// parse_create_table Rust implementation
pub fn rust_parse_create_table(sql: &str) -> RustAst {
    let dialect = GenericDialect {}; // or MySqlDialect, PostgreSqlDialect, etc.
    let ast = Parser::parse_sql(&dialect, sql).expect("Failed to parse SQL");
    let mut table_vec = Vec::new();
    for stmt in ast {
        if let Statement::CreateTable(create_table) = stmt {
            let table_name = &create_table.name.0[0].value;
            let struct_name = Ident::new(&capitalize_first_letter(table_name), proc_macro2::Span::call_site());

            // Create the Rust struct fields
            let mut struct_fields = Vec::new();
            
            for column in create_table.columns {
                let col_name = &column.name.value;
                let col_type = &column.data_type;
                let is_nullable = column.options.iter().any(|opt| opt.option == ColumnOption::Null);

                // Map SQL type to Rust type
                let rust_type = sql_to_type(col_type, is_nullable);

                // Generate the field
                let field_name = Ident::new(&col_name.to_lowercase(), proc_macro2::Span::call_site());
                let field: Field = syn::parse_quote! {
                    pub #field_name: #rust_type
                };
                struct_fields.push(field);
            }
            // Generate the full struct definition using syn
            table_vec.push(ItemStruct {
                attrs: Vec::new(),
                vis: Visibility::Public(Pub::default()),
                struct_token: Default::default(),
                ident: struct_name,
                generics: Default::default(),
                fields: syn::Fields::Named(syn::FieldsNamed {
                    brace_token: Default::default(),
                    named: struct_fields.into_iter().collect(),
                }),
                semi_token: None,
            });
        }
    } // End For statements
    RustAst(table_vec)
}


/// ast_to_string Rust implementation
pub fn rust_ast_to_string(ast: &Vec<ItemStruct>) -> String {
    let mut s = String::new();
    for item in ast {
        s = format!("{}{}", s, item.to_token_stream().to_string());
    }
    s
}

/// sql_to_type Rust implementation
fn sql_to_type(data_type: &DataType, is_nullable: bool) -> syn::Type {
    let base_type: Type = match data_type {
        DataType::Int(_) => syn::parse_quote!(i64),
        DataType::Integer(_) => syn::parse_quote!(i64),
        DataType::Varchar(_) | DataType::Text => syn::parse_quote!(String),
        DataType::Boolean => syn::parse_quote!(bool),
        DataType::Float(_) | DataType::Double => syn::parse_quote!(f64),
        _ => syn::parse_quote!(String), // Default to String for unhandled types
    };
    
    if is_nullable {
        syn::parse_quote!(Option<#base_type>)
    } else {
        base_type
    }
}

/// Return first letter capitalized
fn capitalize_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}