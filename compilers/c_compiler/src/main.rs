use clap::{ArgGroup, Parser};
mod lexer;
mod tokens;
mod parser;
mod parse_ast;

use std::fs;
use std::path::PathBuf;
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
    if args.lex {
        println!("Running lexer...");

        // SS: read the file into a String
        let source_code = fs::read_to_string(&args.input).unwrap_or_else(|err| {
            eprintln!("Error reading file {:?}: {}", args.input, err);
            process::exit(1);
        });

        // SS: pass the string to your Lexer
        let mut lexer = lexer::Lexer::new(source_code);

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
    }
}
