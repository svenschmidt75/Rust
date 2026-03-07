use clap::{ArgGroup, Parser};
mod assembly_ast;
mod emitter;
mod file_emitter;
mod ir_generation;
mod lexer;
mod parse_ast;
mod parser;
mod reg;
mod tokens;
mod x64_code_gen;
mod string_emitter;

use crate::emitter::Emitter;
use crate::file_emitter::FileEmitter;
use crate::x64_code_gen::X64CodeGen;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;

#[derive(Parser)]
#[command(name = "compiler-driver", version = "1.0")]
#[command(group(
    ArgGroup::new("stage")
        .args(["lex", "parse", "codegen", "s"])
        .multiple(false) // Ensures only one can be picked
))]
struct Args {
    /// The C source file to compile
    input: PathBuf,

    /// Run the lexer and stop
    #[arg(long, conflicts_with = "parse")]
    lex: bool,

    /// Run the parser and stop
    #[arg(long)]
    parse: bool,

    /// Stop after assembly generation
    #[arg(long)]
    codegen: bool,

    /// Emit assembly file but do not link
    #[arg(short = 'S')]
    s: bool,
}

fn main() {
    // Only pause if we explicitly ask for it
    if std::env::var("WAIT_FOR_DEBUGGER").is_ok() {
        println!(
            "Wait! I'm PID {}. Attach RustRover now and press Enter...",
            std::process::id()
        );
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
    }

    let args = Args::parse();
    println!("Processing: {:?}", args.input);

    println!("Running lexer...");

    // SS: read the file into a String
    let source_code = fs::read_to_string(&args.input).unwrap_or_else(|err| {
        eprintln!("Error reading file {:?}: {}", args.input, err);
        process::exit(1);
    });

    // SS: pass the string to the lexer
    let mut lexer = lexer::Lexer::new(source_code);

    if args.lex {
        // SS: iterate through tokens until EOF
        loop {
            match lexer.next_token() {
                Ok(tokens::Tokens::EOF) => {
                    println!("Reached end of file.");
                    break;
                }
                Ok(token) => println!("Token: {:?}", token),
                Err(err) => {
                    eprintln!("Lexer error: {}", err);
                    process::exit(1);
                }
            }
        }
    } else {
        println!("Running parser...");
        let mut parser = parser::Parser::new(lexer);
        match parser.parse() {
            Ok(ast) => {
                println!("Parsed AST: {:?}", ast);

                let assembly_ast = ir_generation::generate_assembly_program_ast(ast);

                if args.codegen {
                    println!("Running codegen...");
                    println!("Generated Assembly AST: {:?}", assembly_ast);
                } else if !args.parse {
                    println!("Emitting assembly...");
                    let input_path = Path::new(&args.input);
                    let output_path = input_path.with_extension("s");
                    let mut file_emitter = FileEmitter::new(Path::new(&output_path))
                        .expect("Failed to create FileEmitter");
                    let mut x86_code_gen = X64CodeGen::new(&mut file_emitter);
                    x86_code_gen.emit(assembly_ast);
                    file_emitter.finish().unwrap();
                }
            }
            Err(err) => {
                eprintln!("Parser error: {}", err);
                process::exit(1);
            }
        }
    }
}
