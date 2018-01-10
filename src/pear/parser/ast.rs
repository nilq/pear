use super::lexer::*;

#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionNode {
    Number(f64),
    Bool(bool),
    Str(String),
    Identifier(String),
    EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Expression(ExpressionNode, TokenPosition);

impl Expression {
    pub fn new(node: ExpressionNode, position: TokenPosition) -> Expression {
        Expression(node, position)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum StatementNode {
    Expression(Expression),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Statement(StatementNode, TokenPosition);

impl Statement {
    pub fn new(node: StatementNode, position: TokenPosition) -> Statement {
        Statement(node, position)
    }
}
