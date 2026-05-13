pub mod ir;
pub mod codegen;
pub mod parser;

use parser::PythonParser;
use codegen::RustGenerator;

pub fn compile(source: &str) -> Result<String, String> {
    let ast = PythonParser::parse(source)?;
    PythonParser::validate(source, &ast).map_err(|errors| errors.join("\n"))?;
    let ir = PythonParser::transform(ast);
    let rust_code = RustGenerator::generate(&ir);
    Ok(rust_code)
}
