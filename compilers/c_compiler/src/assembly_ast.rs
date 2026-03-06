use crate::reg::Register;

#[derive(Debug, Clone, PartialEq)]
pub struct AssemblyProgramAST {
    pub function_definition: AssemblyFunctionAST,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AssemblyFunctionAST {
    pub name: String,
    pub instructions: Vec<AssemblyInstructionAST>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AssemblyInstructionAST {
    Mov {
        src: AssemblyOperandAST,
        dst: AssemblyOperandAST,
    },
    Ret,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AssemblyOperandAST {
    Immediate(i64),
    Register(Register),
}
