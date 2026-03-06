#[derive(Debug, Clone, PartialEq)]
pub struct ProgramAST {
    pub function_definition: FunctionAST,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionAST {
    pub name: String,
    pub body: StmtAST,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StmtAST {
    Return(ExprAST),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExprAST {
    Constant(i64),
}
