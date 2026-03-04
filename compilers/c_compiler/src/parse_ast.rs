#[derive(Debug, Clone, PartialEq)]
pub enum ParseAst {
    Program(Box<ParseAst>),

    FunctionDefinition { name: String, body: Box<ParseAst> },

    // SS: statement
    Return(Box<ParseAst>),

    // SS: expression
    Constant(usize),
}
