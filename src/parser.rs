use rustpython_parser::{Parse, ast};
use crate::ir::IR;

pub struct PythonParser;

impl PythonParser {
    pub fn parse(source: &str) -> Result<ast::Suite, String> {
        ast::Suite::parse(source, "input.py")
            .map_err(|e| format!("Parsing error: {}", e))
    }
}
