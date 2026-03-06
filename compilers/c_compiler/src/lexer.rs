use crate::tokens::Tokens;
use regex::Regex;
use std::sync::LazyLock;

// SS: master regex
static MASTER_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"(?P<int>^int\b)|",
        r"(?P<void>^void\b)|",
        r"(?P<return>^return\b)|",
        r"(?P<open_paren>^\()|",
        r"(?P<close_paren>^\))|",
        r"(?P<open_brace>^\{)|",
        r"(?P<close_brace>^\})|",
        r"(?P<semicolon>^;)|",
        r"(?P<identifier>^[a-zA-Z_]\w*\b)|",
        r"(?P<constant>^[0-9]+\b)"
    ))
    .unwrap()
});

pub(crate) struct Lexer {
    input: String,
    position: usize,
    pub current_line: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            input,
            position: 0,
            current_line: 1,
        }
    }

    pub fn next_token(&mut self) -> Result<Tokens, String> {
        self.skip_whitespace();

        if self.position >= self.input.len() {
            return Ok(Tokens::EOF);
        }

        let remaining = &self.input[self.position..];

        if let Some(caps) = MASTER_RE.captures(remaining) {
            // Find which named group matched
            if caps.name("int").is_some() {
                self.position += 3;
                return Ok(Tokens::Int);
            } else if caps.name("void").is_some() {
                self.position += 4;
                return Ok(Tokens::Void);
            } else if caps.name("return").is_some() {
                self.position += 6;
                return Ok(Tokens::Return);
            } else if let Some(mat) = caps.name("identifier") {
                self.position += mat.end();
                return Ok(Tokens::Identifier(mat.as_str().to_string()));
            } else if let Some(mat) = caps.name("constant") {
                let val = mat.as_str().parse::<i64>().unwrap();
                self.position += mat.end();
                return Ok(Tokens::Constant(val));
            }

            // SS: final catch-all for punctuation
            let mat = caps.get(0).unwrap();
            self.position += mat.end();

            return match mat.as_str() {
                "(" => Ok(Tokens::OpenParen),
                ")" => Ok(Tokens::CloseParen),
                "{" => Ok(Tokens::OpenBrace),
                "}" => Ok(Tokens::CloseBrace),
                ";" => Ok(Tokens::Semicolon),
                _ => unreachable!(),
            };
        }

        Err(format!("Line {}: Unexpected character", self.current_line))
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.input[self.position..].chars().next() {
            if c.is_whitespace() {
                if c == '\n' {
                    self.current_line += 1;
                }
                self.position += c.len_utf8();
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::tokens::Tokens;

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
