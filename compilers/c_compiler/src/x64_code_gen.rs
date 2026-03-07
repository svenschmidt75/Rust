use crate::assembly_ast::{
    AssemblyFunctionAST, AssemblyInstructionAST, AssemblyOperandAST, AssemblyProgramAST,
};
use crate::emitter::Emitter;

pub(crate) struct X64CodeGen<'a, E: Emitter> {
    emitter: &'a mut E,
}

impl<'a, E: Emitter> X64CodeGen<'a, E> {
    pub fn new(emitter: &'a mut E) -> Self {
        X64CodeGen { emitter }
    }

    pub fn emit(&mut self, ast: &AssemblyProgramAST) {
        self.emitter.emit(".globl _main");
        self.emit_function(&ast.function_definition);
    }

    fn emit_function(&mut self, function_ast: &AssemblyFunctionAST) {
        self.emitter.emit("_main:");
        for instruction in &function_ast.instructions {
            self.emit_instruction(&instruction);
        }
    }

    fn emit_instruction(&mut self, instruction: &AssemblyInstructionAST) {
        match instruction {
            AssemblyInstructionAST::Mov { src, dst } => {
                let src_str = self.emit_operand(src);
                let dst_str = self.emit_operand(dst);
                self.emitter
                    .emit(&format!("    movl {}, {}", src_str, dst_str));
            }
            AssemblyInstructionAST::Ret => {
                self.emitter.emit("    ret");
            }
        }
    }

    fn emit_operand(&self, operand: &AssemblyOperandAST) -> String {
        match operand {
            AssemblyOperandAST::Immediate(val) => format!("${}", val),
            AssemblyOperandAST::Register(reg) => format!("%{}", reg),
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
        let assembly_ast = assembly_ast::AssemblyProgramAST {
            function_definition: assembly_ast::AssemblyFunctionAST {
                name: "main".to_string(),
                instructions: vec![
                    assembly_ast::AssemblyInstructionAST::Mov {
                        src: assembly_ast::AssemblyOperandAST::Immediate(2),
                        dst: assembly_ast::AssemblyOperandAST::Register(crate::reg::Register::EAX),
                    },
                    assembly_ast::AssemblyInstructionAST::Ret,
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
