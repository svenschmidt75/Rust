use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Register {
    EAX,
}

impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Register::EAX => "eax".to_string(),
            }
        )
    }
}
