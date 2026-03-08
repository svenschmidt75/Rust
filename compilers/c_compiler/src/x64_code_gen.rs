use crate::assembly_ast::{
    FunctionAST, InstructionAST, OperandAST, ProgramAST,
};
use crate::emitter::Emitter;

pub(crate) struct X64CodeGen<'a, E: Emitter> {
    emitter: &'a mut E,
}

impl<'a, E: Emitter> X64CodeGen<'a, E> {
    pub fn new(emitter: &'a mut E) -> Self {
        X64CodeGen { emitter }
    }

    pub fn emit(&mut self, ast: &ProgramAST) {
        self.emitter.emit(".globl _main");
        self.emit_function(&ast.function_definition);
    }

    fn emit_function(&mut self, function_ast: &FunctionAST) {
        self.emitter.emit("_main:");
        for instruction in &function_ast.instructions {
            self.emit_instruction(&instruction);
        }
    }

    fn emit_instruction(&mut self, instruction: &InstructionAST) {
        match instruction {
            InstructionAST::Mov { src, dst } => {
                let src_str = self.emit_operand(src);
                let dst_str = self.emit_operand(dst);
                self.emitter
                    .emit(&format!("    movl {}, {}", src_str, dst_str));
            }
            InstructionAST::Ret => {
                self.emitter.emit("    ret");
            }
        }
    }

    fn emit_operand(&self, operand: &OperandAST) -> String {
        match operand {
            OperandAST::Immediate(val) => format!("${}", val),
            OperandAST::Register(reg) => format!("%{}", reg),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::assembly_ast;
    use crate::string_emitter::StringEmitter;
    use crate::x64_code_gen::X64CodeGen;

    #[test]
    fn test_x64_code_gen() {
        // SS: arrange
        let assembly_ast = assembly_ast::ProgramAST {
            function_definition: assembly_ast::FunctionAST {
                name: "main".to_string(),
                instructions: vec![
                    assembly_ast::InstructionAST::Mov {
                        src: assembly_ast::OperandAST::Immediate(2),
                        dst: assembly_ast::OperandAST::Register(crate::reg::Register::EAX),
                    },
                    assembly_ast::InstructionAST::Ret,
                ],
            },
        };

        // SS: act
        let mut emitter = StringEmitter::new();
        let mut code_gen = X64CodeGen::new(&mut emitter);
        code_gen.emit(&assembly_ast);

        // SS: assert
        let emitted_code = emitter.buffer;
        assert!(emitted_code.contains(&"    movl $2, %eax".to_string()));
        assert!(emitted_code.contains(&"    ret".to_string()));
    }
}
