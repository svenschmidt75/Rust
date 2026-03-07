use crate::assembly_ast::{
    AssemblyFunctionAST, AssemblyInstructionAST, AssemblyOperandAST, AssemblyProgramAST,
};
use crate::parse_ast::{ExprAST, FunctionAST, ProgramAST, StmtAST};

pub fn generate_assembly_program_ast(parse_ast: ProgramAST) -> AssemblyProgramAST {
    let ProgramAST {
        function_definition,
    } = parse_ast;

    AssemblyProgramAST {
        function_definition: generate_assembly_function_ast(function_definition),
    }
}

fn generate_assembly_function_ast(function_definition: FunctionAST) -> AssemblyFunctionAST {
    let FunctionAST { name, body } = function_definition;
    AssemblyFunctionAST {
        name,
        instructions: generate_assembly_instructions_ast(body),
    }
}

fn generate_assembly_instructions_ast(stmt: StmtAST) -> Vec<AssemblyInstructionAST> {
    match stmt {
        StmtAST::Return(expr) => vec![
            AssemblyInstructionAST::Mov {
                src: generate_assembly_expr_ast(expr),
                dst: AssemblyOperandAST::Register(crate::reg::Register::EAX),
            },
            AssemblyInstructionAST::Ret,
        ],
    }
}

fn generate_assembly_expr_ast(expr: ExprAST) -> AssemblyOperandAST {
    match expr {
        ExprAST::Constant(val) => AssemblyOperandAST::Immediate(val),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_generate_assembly_program_ast() {
        // SS: arrange
        let parse_ast = crate::parse_ast::ProgramAST {
            function_definition: crate::parse_ast::FunctionAST {
                name: "main".to_string(),
                body: crate::parse_ast::StmtAST::Return(crate::parse_ast::ExprAST::Constant(2)),
            },
        };

        // SS: act
        let assembly_ast = crate::ir_generation::generate_assembly_program_ast(parse_ast);

        // SS: assert
        assert_eq!(
            assembly_ast,
            crate::assembly_ast::AssemblyProgramAST {
                function_definition: crate::assembly_ast::AssemblyFunctionAST {
                    name: "main".to_string(),
                    instructions: vec![
                        crate::assembly_ast::AssemblyInstructionAST::Mov {
                            src: crate::assembly_ast::AssemblyOperandAST::Immediate(2),
                            dst: crate::assembly_ast::AssemblyOperandAST::Register(
                                crate::reg::Register::EAX
                            ),
                        },
                        crate::assembly_ast::AssemblyInstructionAST::Ret,
                    ],
                }
            }
        );
    }
}
