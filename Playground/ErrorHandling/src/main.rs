use std::env;
use std::env::VarError;

#[derive(Debug)]
enum Errors {
    Env(std::env::VarError),
    Parse(std::num::ParseIntError),
}

impl std::convert::From<std::env::VarError> for Errors {
    fn from(e: VarError) -> Self {
        Errors::Env(e)
    }
}

impl std::convert::From<std::num::ParseIntError> for Errors {
    fn from(e: std::num::ParseIntError) -> Self {
        Errors::Parse(e)
    }
}

fn num_threads() -> Result<usize, Errors> {
    let s = env::var("NUM_THREADS")?;
    let n: usize = s.parse()?;
    Ok(n + 1)
}



fn main() {
    let num_threads = num_threads();
    match num_threads {
        Ok(n) => println!("Num threads: {}", n),
        Err(e) => println!("{:?}", e),
    }

    println!("Hello, world!");
}
