#[derive(Debug, Clone, PartialEq)]
pub struct ProgramAST {
    pub function_definition: FunctionAST,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionAST {
    pub name: String,
    pub body: Vec<InstructionAST>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InstructionAST {
    Return(ValAST),
    Unary {
        operator: UnaryOperatorAST,
        src: ValAST,
        dst: ValAST,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum ValAST {
    Constant(i64),
    Variable(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperatorAST {
    Complement,
    Negate,
}
