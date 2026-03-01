use crate::tokens::Tokens;
use regex::Regex;

pub(crate) struct Lexer {
    rules: Vec<(Regex, Tokens)>,
    input: String,
    position: usize,
    current_line: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            // SS: we have to check for keywords first, otherwise they will be matched as identifiers
            rules: vec![
                (Regex::new(r"int\b").unwrap(), Tokens::Int),
                (Regex::new(r"void\b").unwrap(), Tokens::Void),
                (Regex::new(r"return\b").unwrap(), Tokens::Return),
                (Regex::new(r"\(").unwrap(), Tokens::OpenParen),
                (Regex::new(r"\)").unwrap(), Tokens::CloseParen),
                (Regex::new(r"\{").unwrap(), Tokens::OpenBrace),
                (Regex::new(r"}").unwrap(), Tokens::CloseBrace),
                (Regex::new(r";").unwrap(), Tokens::Semicolon),
                (
                    Regex::new(r"[a-zA-Z_]\w*\b").unwrap(),
                    Tokens::Identifier(String::new()),
                ),
                (Regex::new(r"[0-9]+\b").unwrap(), Tokens::Constant(0)),
            ],
            input,
            position: 0,
            current_line: 1,
        }
    }

    pub fn next_token(&mut self) -> Result<Tokens, String> {
        while self.position < self.input.len() {
            let c = &self.input.chars().nth(self.position).unwrap();
            if c.is_whitespace() {
                if *c == '\n' {
                    self.current_line += 1;
                }
                self.position += 1;
                continue;
            }

            // SS: check all rules
            for (pattern, token) in &self.rules {
                if let Some(mat) = pattern.find(&self.input[self.position..]) {
                    if mat.start() == 0 {
                        self.position += mat.end();

                        return match token {
                            Tokens::Identifier(_) => {
                                let identifier = self.input
                                    [self.position - mat.end()..self.position]
                                    .to_string();
                                return Ok(Tokens::Identifier(identifier));
                            }
                            Tokens::Constant(_) => {
                                let constant = self.input[self.position - mat.end()..self.position]
                                    .parse::<usize>()
                                    .map_err(|e| {
                                        format!(
                                            "Line {}: Failed to parse constant: {}",
                                            self.current_line, e
                                        )
                                    })?;
                                return Ok(Tokens::Constant(constant));
                            }
                            _ => Ok(token.clone()),
                        };
                    }
                }
            }

            return Err(format!(
                "Line {}: Unexpected character {}",
                self.current_line,
                self.input[self.position..].to_string()
            ));
        }

        Ok(Tokens::EOF)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        // SS: arrange
        let input = r"int main(void) {
                            return 2;
                    }"
        .to_string();

        // SS: act
        let mut lexer = Lexer::new(input);

        // SS: assert
        assert_eq!(lexer.next_token().unwrap(), Tokens::Int);
        assert_eq!(
            lexer.next_token().unwrap(),
            Tokens::Identifier("main".to_string())
        );
        assert_eq!(lexer.next_token().unwrap(), Tokens::OpenParen);
        assert_eq!(lexer.next_token().unwrap(), Tokens::Void);
        assert_eq!(lexer.next_token().unwrap(), Tokens::CloseParen);
        assert_eq!(lexer.next_token().unwrap(), Tokens::OpenBrace);
        assert_eq!(lexer.next_token().unwrap(), Tokens::Return);
        assert_eq!(lexer.next_token().unwrap(), Tokens::Constant(2));
        assert_eq!(lexer.next_token().unwrap(), Tokens::Semicolon);
        assert_eq!(lexer.next_token().unwrap(), Tokens::CloseBrace);
        assert_eq!(lexer.next_token().unwrap(), Tokens::EOF);
    }
}
