use crate::parse_ast::UnaryOperatorAST;
use crate::tacky_ast::InstructionAST::Unary;
use crate::{parse_ast, tacky_ast};

pub struct TackyGenerator {
    temp_counter: usize,
    instructions: Vec<tacky_ast::InstructionAST>,
}

impl TackyGenerator {
    pub fn new() -> Self {
        Self {
            temp_counter: 0,
            instructions: Vec::new(),
        }
    }

    pub fn generate_program_ast(
        &mut self,
        parse_ast: parse_ast::ProgramAST,
    ) -> tacky_ast::ProgramAST {
        let parse_ast::ProgramAST {
            function_definition,
        } = parse_ast;

        tacky_ast::ProgramAST {
            function_definition: self.generate_function_ast(function_definition),
        }
    }

    fn generate_function_ast(
        &mut self,
        function_definition: parse_ast::FunctionAST,
    ) -> tacky_ast::FunctionAST {
        // SS: clear instructions before generating new function AST
        self.instructions.clear();

        let parse_ast::FunctionAST { name, body } = function_definition;

        // SS: emits instructions for the function body, which will be stored in self.instructions
        self.generate_instructions_ast(body);

        tacky_ast::FunctionAST {
            name,
            body: self.instructions.clone(),
        }
    }

    fn generate_instructions_ast(&mut self, stmt: parse_ast::StmtAST) {
        match stmt {
            parse_ast::StmtAST::Return(expr) => {
                // SS: generate TACHY instructions for the expression
                let expr_ast = self.generate_expr_ast(expr);

                // SS: emit a return instruction with the computed expression value
                self.instructions
                    .push(tacky_ast::InstructionAST::Return(expr_ast));
            }
        }
    }

    fn generate_expr_ast(&mut self, expr: parse_ast::ExprAST) -> tacky_ast::ValAST {
        // SS: We're transforming high-level expressions into a sequence of instructions
        // that compute the value of the expression. As such, one expr might generate multiple
        // instructions if it involves operations that require temporary variables.
        match expr {
            parse_ast::ExprAST::Constant(val) => tacky_ast::ValAST::Constant(val),
            parse_ast::ExprAST::Unary(op, inner_expr) => {
                let inner_expr_ast = self.generate_expr_ast(*inner_expr);

                // SS: destination variable with result of unary operation
                let dst = self.make_temporary();

                let unary_ast = Unary {
                    operator: match op {
                        UnaryOperatorAST::Complement => tacky_ast::UnaryOperatorAST::Complement,
                        UnaryOperatorAST::Negate => tacky_ast::UnaryOperatorAST::Negate,
                    },
                    src: inner_expr_ast,
                    dst: dst.clone(),
                };
                self.instructions.push(unary_ast);

                dst
            }
        }
    }

    fn make_temporary(&mut self) -> tacky_ast::ValAST {
        let name = format!("tmp.{}", self.temp_counter);
        self.temp_counter += 1;
        tacky_ast::ValAST::Variable(name)
    }
}
