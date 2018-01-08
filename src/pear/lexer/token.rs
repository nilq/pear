#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Number,
    Str,
    Bool,
    Symbol,
    Operator,
    Identifier,
    Keyword,
    Whitespace,
    EOL,
    EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TokenPosition(usize, usize);

impl TokenPosition {
    pub fn new(line: usize, col: usize) -> Self {
        TokenPosition(line, col)
    }
}

impl Default for TokenPosition {
    fn default() -> Self {
        TokenPosition(1, 1)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token<'t> {
    pub kind:     TokenType,
    pub position: TokenPosition,
    pub lexeme:   &'t str,
    pub line:     &'t str,
}

impl<'t> Token<'t> {
    pub fn new(kind: TokenType, position: TokenPosition, lexeme: &'t str, line: &'t str) -> Self {
        Token {
            kind,
            position,
            lexeme,
            line,
        }
    }
}
