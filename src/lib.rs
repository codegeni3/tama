pub fn compile(source: &str) -> Result<String, String> {
    Ok(format!("// Mock compiled output of:
{}", source))
}
