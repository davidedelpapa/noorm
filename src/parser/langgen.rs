use serde::Deserialize;
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;
use sqlparser::ast::{Statement, DataType, ColumnOption};

/// Trait to handle the generators in various target languages
pub trait LanguageGenerator {
    fn parse_create_table(&self, sql: &str) -> String;
    fn sql_to_type(&self, data_type: &DataType, is_nullable: bool) -> String;
}

/// Rust target generator
#[derive(Debug, Default, PartialEq, Clone, Deserialize)]
pub struct RustGenerator;

/// Python target generator
#[derive(Debug, Default, PartialEq, Clone, Deserialize)]
pub struct PythonGenerator;

/// JavaScript target generator
#[derive(Debug, Default, PartialEq, Clone, Deserialize)]
pub struct JavaScriptGenerator;

impl LanguageGenerator for RustGenerator {
    /// parse_create_table Rust implementation
    fn parse_create_table(&self, sql: &str) -> String {
        let dialect = GenericDialect {}; // or MySqlDialect, PostgreSqlDialect, etc.
        let ast = Parser::parse_sql(&dialect, sql).expect("Failed to parse SQL");

        for stmt in ast {
            if let Statement::CreateTable(create_table) = stmt {
                let table_name = &create_table.name.0[0].value;
                
                // Start generating Rust struct code
                let mut rust_fields = String::new();
                
                for column in create_table.columns {
                    let col_name = &column.name.value;
                    let col_type = &column.data_type;
                    let is_nullable = column.options.iter().any(|opt| opt.option == ColumnOption::Null);

                    // Map SQL type to Rust type
                    let rust_type = self.sql_to_type(col_type, is_nullable);

                    // Add the field to the Rust struct string
                    rust_fields.push_str(&format!("    pub {}: {},\n", col_name.to_lowercase(), rust_type));
                }

                // Capitalize the first letter of the table name for Rust struct
                let struct_name = format!(
                    "{}{}",
                    table_name[..1].to_uppercase(),
                    &table_name[1..].to_lowercase()
                );

                // Return the final Rust struct string
                return format!(
                    "pub struct {} {{\n{}}}",
                    struct_name, rust_fields
                );
            }
        } // End For

        String::new()
    }
    
    /// sql_to_type Rust implementation
    fn sql_to_type(&self, data_type: &DataType, is_nullable: bool) -> String {
        let rust_type = match data_type {
            DataType::Int(_) => "i64".to_string(),
            DataType::Integer(_) => "i64".to_string(),
            DataType::Varchar(_) | DataType::Text => "String".to_string(),
            DataType::Boolean => "bool".to_string(),
            DataType::Float(_) | DataType::Double => "f64".to_string(),
            _ => "String".to_string(), // Default to String for unhandled types
        };
        
        if is_nullable {
            format!("Option<{}>", rust_type)
        } else {
            rust_type
        }
    }
}

impl LanguageGenerator for PythonGenerator {
    /// parse_create_table Python implementation
    fn parse_create_table(&self, _sql: &str) -> String {
        "print(\"Hello, world!\")".to_string()
    }
    
    /// sql_to_type Python implementation
    fn sql_to_type(&self, _data_type: &DataType, _is_nullable: bool) -> String {
        "print(\"Hello, world!\")".to_string()
    }
}

impl LanguageGenerator for JavaScriptGenerator {
    /// parse_create_table JavaScript implementation
    fn parse_create_table(&self, _sql: &str) -> String {
        "console.log(\"Hello, world!\");".to_string()
    }
    
    /// sql_to_type JavaScript implementation
    fn sql_to_type(&self, _data_type: &DataType, _is_nullable: bool) -> String {
        "console.log(\"Hello, world!\");".to_string()
    }
}

