use crate::lexer::Lexer;
use crate::parse_ast::ParseAst;
use crate::tokens::Tokens;

struct Parser {
    lexer: Lexer,
    current_symbol: Option<Tokens>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Parser {
            lexer,
            current_symbol: None,
        }
    }

    pub fn parse(&mut self) -> Result<Box<ParseAst>, String> {
        let ast = self.parse_function_definition()?;

        // SS: ensure we have consumed all tokens
        if self.advance()? != Tokens::EOF {
            return Err(format!(
                "Line {}: Syntax error: Expected end of file, but found {:?}",
                self.lexer.current_line,
                self.current_symbol.as_ref().unwrap().to_string()
            ));
        }

        Ok(Box::new(ParseAst::Program(ast)))
    }

    pub(crate) fn parse_function_definition(&mut self) -> Result<Box<ParseAst>, String> {
        self.expect("int", Tokens::Int)?;

        // SS: parse the function name
        let Tokens::Identifier(name) = self.advance()? else {
            return Err(format!(
                "Line {}: Syntax error: Expected function name, but found {:?}",
                self.lexer.current_line,
                self.current_symbol.as_ref().unwrap().to_string()
            ));
        };

        self.expect("(", Tokens::OpenParen)?;
        self.expect("void", Tokens::Void)?;
        self.expect(")", Tokens::CloseParen)?;
        self.expect("{", Tokens::OpenBrace)?;

        let stmt = self.parse_stmt()?;

        self.expect("}", Tokens::CloseBrace)?;

        Ok(Box::new(ParseAst::FunctionDefinition {
            name: name.to_string(),
            body: stmt,
        }))
    }

    fn parse_stmt(&mut self) -> Result<Box<ParseAst>, String> {
        self.expect("return", Tokens::Return)?;
        let expr = self.parse_expr()?;
        self.expect(";", Tokens::Semicolon)?;
        Ok(Box::new(ParseAst::Return(expr)))
    }

    fn parse_expr(&mut self) -> Result<Box<ParseAst>, String> {
        let Tokens::Constant(val) = self.advance()? else {
            return Err(format!(
                "Line {}: Syntax error: Expected constant, but found {:?}",
                self.lexer.current_line,
                self.current_symbol.as_ref().unwrap().to_string()
            ));
        };
        Ok(Box::new(ParseAst::Constant(val)))
    }

    fn expect(&mut self, expected_string: &str, token: Tokens) -> Result<Tokens, String> {
        let actual = self.advance()?;
        if actual == token {
            Ok(actual)
        } else {
            Err(format!(
                "Line {}: Syntax error: Expected token {:?}, but found {:?}",
                self.lexer.current_line, expected_string, actual
            ))
        }
    }

    fn peek(&mut self) -> Result<Tokens, String> {
        if self.current_symbol.is_none() {
            self.current_symbol = Some(self.lexer.next_token()?);
        }
        Ok(self.current_symbol.clone().unwrap())
    }

    fn advance(&mut self) -> Result<Tokens, String> {
        let token = if let Some(t) = self.current_symbol.take() {
            t
        } else {
            self.lexer.next_token()?
        };
        Ok(token)
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::parse_ast::ParseAst;
    use crate::parser::Parser;

    #[test]
    fn test_parser() {
        // SS: arrange
        let input = r"int main(void) {
                            return 2;
                    }"
        .to_string();

        // SS: act
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let ast = parser.parse().unwrap();

        // SS: assert
        assert_eq!(
            *ast,
            ParseAst::Program(Box::new(ParseAst::FunctionDefinition {
                name: "main".to_string(),
                body: Box::new(ParseAst::Return(Box::new(ParseAst::Constant(2)))),
            }))
        );
    }

    #[test]
    fn test_parser_fail() {
        // SS: arrange
        let input = r"inta main(void) {
                            return 2;
                    }"
        .to_string();

        // SS: act
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let ast = parser.parse();

        // SS: assert
        assert_eq!(
            ast,
            Err(r#"Line 1: Syntax error: Expected token "int", but found Identifier("inta")"#.to_string())
        );
    }
}
