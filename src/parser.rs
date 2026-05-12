use rustpython_parser::{Parse, ast};
use rustpython_parser::ast::Ranged;
use num_traits::ToPrimitive;
use crate::ir::IR;

pub struct PythonParser;

impl PythonParser {
    pub fn parse(source: &str) -> Result<ast::Suite, String> {
        ast::Suite::parse(source, "input.py")
            .map_err(|e| format!("Parsing error: {}", e))
    }

    pub fn validate(source: &str, ast: &ast::Suite) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        for stmt in ast {
            validate_stmt(source, stmt, &mut errors);
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

fn offset_to_line(source: &str, offset: usize) -> usize {
    let mut line = 1;
    for (i, c) in source.char_indices() {
        if i >= offset {
            break;
        }
        if c == n {
            line += 1;
        }
    }
    line
}

fn validate_function_body(source: &str, body: &[ast::Stmt], errors: &mut Vec<String>) {
    for stmt in body {
        validate_stmt(source, stmt, errors);
    }
}

fn validate_stmt(source: &str, stmt: &ast::Stmt, errors: &mut Vec<String>) {
    let line = offset_to_line(source, stmt.range().start().to_usize());
    match stmt {
        ast::Stmt::FunctionDef(f) => {
            for decorator in &f.decorator_list {
                let dec_name = get_call_name(decorator);
                if dec_name != "tama.contract" && dec_name != "contract" {
                    errors.push(format!("Line {}: Unsupported decorator: {}", line, dec_name));
                }
            }
            validate_function_body(source, &f.body, errors);
        }
        ast::Stmt::Assign(a) => {
            validate_expr(source, &a.value, errors);
        }
        ast::Stmt::Expr(e) => {
            validate_expr(source, &e.value, errors);
        }
        ast::Stmt::Return(r) => {
            if let Some(value) = &r.value {
                validate_expr(source, value, errors);
            }
        }
        ast::Stmt::Import(imp) => {
            let mut allowed = false;
            if imp.names.len() == 1 {
                if imp.names[0].name.to_string() == "tama" {
                    allowed = true;
                }
            }
            if !allowed {
                errors.push(format!("Line {}: Imports are not supported", line));
            }
        }
        ast::Stmt::ImportFrom(imp) => {
            let mut allowed = false;
            if let Some(module) = &imp.module {
                if module.to_string() == "tama" {
                    allowed = true;
                }
            }
            if !allowed {
                errors.push(format!("Line {}: Imports are not supported", line));
            }
        }
        ast::Stmt::ClassDef(_) => {
            errors.push(format!("Line {}: Classes are not supported", line));
        }
        ast::Stmt::AsyncFunctionDef(_) => {
            errors.push(format!("Line {}: Async functions are not supported", line));
        }
        other => {
            errors.push(format!("Line {}: Unsupported statement type: {:?}", line, other));
        }
    }
}

fn validate_expr(source: &str, expr: &ast::Expr, errors: &mut Vec<String>) {
    let line = offset_to_line(source, expr.range().start().to_usize());
    match expr {
        ast::Expr::Constant(c) => {
            match &c.value {
                ast::Constant::Int(_) | ast::Constant::Str(_) => {}
                _ => {
                    errors.push(format!("Line {}: Only integer and string literals are supported", line));
                }
            }
        }
        ast::Expr::Name(_) => {}
        ast::Expr::BinOp(b) => {
            validate_expr(source, &b.left, errors);
            validate_expr(source, &b.right, errors);
            if !matches!(b.op, ast::Operator::Add) {
                errors.push(format!("Line {}: Unsupported operator, only + is supported", line));
            }
        }
        ast::Expr::Call(c) => {
            let call_name = get_call_name(&c.func);
            let mut is_allowed_call = false;
            if call_name == "storage.get" || call_name == "storage.set" || call_name == "tama.storage.get" || call_name == "tama.storage.set" {
                is_allowed_call = true;
            }
            if !is_allowed_call {
                errors.push(format!("Line {}: Unsupported function call: {}", line, call_name));
            }
            for arg in &c.args {
                validate_expr(source, arg, errors);
            }
        }
        other => {
            errors.push(format!("Line {}: Unsupported expression type: {:?}", line, other));
        }
    }
}

fn get_call_name(expr: &ast::Expr) -> String {
    match expr {
        ast::Expr::Name(n) => n.id.to_string(),
        ast::Expr::Attribute(a) => {
            let base = get_call_name(&a.value);
            if base.is_empty() {
                a.attr.to_string()
            } else {
                format!("{}.{}", base, a.attr)
            }
        }
        _ => String::new(),
    }
}
