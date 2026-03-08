use crate::reg::Register;

#[derive(Debug, Clone, PartialEq)]
pub struct ProgramAST {
    pub function_definition: FunctionAST,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionAST {
    pub name: String,
    pub instructions: Vec<InstructionAST>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InstructionAST {
    Mov {
        src: OperandAST,
        dst: OperandAST,
    },
    Ret,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OperandAST {
    Immediate(i64),
    Register(Register),
}
