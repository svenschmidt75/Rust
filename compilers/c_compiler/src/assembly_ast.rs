use crate::reg::Register;

#[derive(Debug, Clone)]
pub struct AssemblyProgramAST {
    pub function_definition: AssemblyFunctionAST,
}

#[derive(Debug, Clone)]
pub struct AssemblyFunctionAST {
    pub name: String,
    pub instructions: Vec<AssemblyInstructionAST>,
}

#[derive(Debug, Clone)]
pub enum AssemblyInstructionAST {
    Mov {
        src: AssemblyOperandAST,
        dst: AssemblyOperandAST,
    },
    Ret,
}

#[derive(Debug, Clone)]
pub enum AssemblyOperandAST {
    Immediate(i64),
    Register(Register),
}
