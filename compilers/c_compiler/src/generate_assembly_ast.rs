use crate::assembly_ast;
use crate::parse_ast;

pub fn generate_program_ast(parse_ast: parse_ast::ProgramAST) -> assembly_ast::ProgramAST {
    let parse_ast::ProgramAST {
        function_definition,
    } = parse_ast;

    assembly_ast::ProgramAST {
        function_definition: generate_function_ast(function_definition),
    }
}

fn generate_function_ast(function_definition: parse_ast::FunctionAST) -> assembly_ast::FunctionAST {
    let parse_ast::FunctionAST { name, body } = function_definition;
    assembly_ast::FunctionAST {
        name,
        instructions: generate_instructions_ast(body),
    }
}

fn generate_instructions_ast(stmt: parse_ast::StmtAST) -> Vec<assembly_ast::InstructionAST> {
    match stmt {
        parse_ast::StmtAST::Return(expr) => vec![
            assembly_ast::InstructionAST::Mov {
                src: generate_expr_ast(expr),
                dst: assembly_ast::OperandAST::Register(crate::reg::Register::EAX),
            },
            assembly_ast::InstructionAST::Ret,
        ],
    }
}

fn generate_expr_ast(expr: parse_ast::ExprAST) -> assembly_ast::OperandAST {
    match expr {
        parse_ast::ExprAST::Constant(val) => assembly_ast::OperandAST::Immediate(val),
        parse_ast::ExprAST::Unary(op, inner_expr) => {
            let operand = generate_expr_ast(*inner_expr);
            match op {
                crate::parse_ast::UnaryOperatorAST::Complement => {
                    // SS: for bitwise complement, we can use the NOT instruction
                    // However, since we are only handling constants in this simple example,
                    // we can compute the complement at compile time.
                    if let assembly_ast::OperandAST::Immediate(val) = operand {
                        assembly_ast::OperandAST::Immediate(!val)
                    } else {
                        panic!("Unexpected non-immediate operand for complement");
                    }
                }
                crate::parse_ast::UnaryOperatorAST::Negate => {
                    // SS: for negation, we can use the NEG instruction
                    // Again, since we are only handling constants, we can compute it at compile time.
                    if let assembly_ast::OperandAST::Immediate(val) = operand {
                        assembly_ast::OperandAST::Immediate(-val)
                    } else {
                        panic!("Unexpected non-immediate operand for negate");
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{assembly_ast, parse_ast, reg};

    #[test]
    fn test_generate_program_ast() {
        // SS: arrange
        let parse_ast = parse_ast::ProgramAST {
            function_definition: parse_ast::FunctionAST {
                name: "main".to_string(),
                body: parse_ast::StmtAST::Return(parse_ast::ExprAST::Constant(2)),
            },
        };

        // SS: act
        let assembly_ast = crate::generate_assembly_ast::generate_program_ast(parse_ast);

        // SS: assert
        assert_eq!(
            assembly_ast,
            assembly_ast::ProgramAST {
                function_definition: crate::assembly_ast::FunctionAST {
                    name: "main".to_string(),
                    instructions: vec![
                        assembly_ast::InstructionAST::Mov {
                            src: assembly_ast::OperandAST::Immediate(2),
                            dst: assembly_ast::OperandAST::Register(reg::Register::EAX),
                        },
                        assembly_ast::InstructionAST::Ret,
                    ],
                }
            }
        );
    }
}
