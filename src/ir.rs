use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IR {
    pub functions: Vec<FunctionDefinition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionDefinition {
    pub name: String,
    pub params: Vec<String>,
    pub body: Vec<Instruction>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Instruction {
    LoadStorage { dest: String, key: String },
    StoreStorage { key: String, value: Operand },
    SetVar { name: String, value: Expression },
    Add { dest: String, left: Operand, right: Operand },
    Return { value: Option<Operand> },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Operand {
    Const(Value),
    Var(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Expression {
    Const(Value),
    Var(String),
    Add(Operand, Operand),
    Call(String, Vec<Operand>),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Value {
    Int(i64),
    String(String),
}
