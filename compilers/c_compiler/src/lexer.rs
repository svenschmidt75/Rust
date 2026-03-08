use crate::tokens::Tokens;
use regex::Regex;
use std::sync::LazyLock;

// SS: master regex
static MASTER_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"(?P<comment>^//.*)|",
        r"(?P<int>^int\b)|",
        r"(?P<void>^void\b)|",
        r"(?P<return>^return\b)|",
        r"(?P<decrement>^--)|",
        r"(?P<negate>^-)|",
        r"(?P<complement>^~)|",
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
        loop {
            self.skip_whitespace();

            if self.position >= self.input.len() {
                return Ok(Tokens::EOF);
            }

            let remaining = &self.input[self.position..];

            return if let Some(caps) = MASTER_RE.captures(remaining) {
                // SS: the first match (index 0) is the whole thing
                let mat = caps.get(0).unwrap();
                let text = mat.as_str();

                // SS: advance position in text by the length of the match
                self.position += mat.end();

                if caps.name("comment").is_some() {
                    // SS: skip comments entirely
                    continue;
                }
                if caps.name("int").is_some() {
                    return Ok(Tokens::Int);
                }
                if caps.name("void").is_some() {
                    return Ok(Tokens::Void);
                }
                if caps.name("return").is_some() {
                    return Ok(Tokens::Return);
                }

                if let Some(m) = caps.name("identifier") {
                    return Ok(Tokens::Identifier(m.as_str().to_string()));
                }

                if let Some(m) = caps.name("constant") {
                    let val = m
                        .as_str()
                        .parse::<i64>()
                        .map_err(|_| "Integer literal too large".to_string())?;
                    return Ok(Tokens::Constant(val));
                }

                // Punctuation
                match text {
                    "~" => Ok(Tokens::Complement),
                    "--" => Ok(Tokens::Decrement),
                    "-" => Ok(Tokens::Negate),
                    "(" => Ok(Tokens::OpenParen),
                    ")" => Ok(Tokens::CloseParen),
                    "{" => Ok(Tokens::OpenBrace),
                    "}" => Ok(Tokens::CloseBrace),
                    ";" => Ok(Tokens::Semicolon),
                    _ => Err(format!(
                        "Line {}: Unrecognized token '{}'",
                        self.current_line, text
                    )),
                }
            } else {
                Err(format!("Line {}: Unexpected character", self.current_line))
            }
        }
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
