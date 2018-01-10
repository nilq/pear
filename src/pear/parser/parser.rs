use super::lexer::*;
use super::*;

pub struct Parser {
    pub tokens: Vec<Token>,
    pub top:    usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            top: 0,
        }
    }
    
    pub fn parse(&mut self) -> ResResult<Vec<Statement>> {
        let mut statements = Vec::new();

        while self.remaining() > 1 {
            statements.push(self.statement()?)
        }

        Ok(statements)
    }
    
    fn statement(&mut self) -> ResResult<Statement> {
        use StatementNode::*;
        
        let node = match self.current_type() {
            _ => Expression(self.expression()?)
        };

        Ok(Statement::new(node, self.position()))
    }

    fn expression(&mut self) -> ResResult<Expression> {
        let expression = self.atom()?;

        Ok(expression)
    }

    fn atom(&mut self) -> ResResult<Expression> {
        use ExpressionNode::*;
        
        self.skip_types(vec![TokenType::EOL, TokenType::Whitespace])?;

        if self.remaining() == 1 {
            return Ok(Expression::new(EOF, self.position()))
        }

        let node = match self.current_type() {
            TokenType::Number     => Number(self.consume_type(TokenType::Number)?.parse().unwrap()),
            TokenType::Str        => Str(self.consume_type(TokenType::Str)?),
            TokenType::Bool       => Bool(self.consume_type(TokenType::Bool)? == "true"),
            TokenType::Identifier => Identifier(self.consume_type(TokenType::Identifier)?),

            t => return Err(make_error(Some(self.position()), format!("token type '{:?}' currently unimplemented", t)))
        };

        Ok(Expression::new(node, self.position()))
    }

    
    fn next(&mut self) -> ResResult<()> {
        if self.top < self.tokens.len() {
            self.top += 1;
            Ok(())
        } else {
            Err(make_error(None, "nexting outside token stack".to_owned()))
        }
    }

    fn back(&mut self) -> ResResult<()> {
        if self.top > 0 {
            self.top -= 1;
            Ok(())
        } else {
            Err(make_error(None, "backing outside token stack".to_owned()))
        }
    }

    fn skip_types(&mut self, tokens: Vec<TokenType>) -> ResResult<()> {
        loop {
            if self.remaining() > 1 {
                if tokens.contains(&self.current_type()) {
                    self.next()?
                } else {
                    break
                }
            } else {
                break
            }
        }

        Ok(())
    }

    fn remaining(&self) -> usize {
        self.tokens.len() - self.top + 1
    }

    pub fn current(&self) -> &Token {
        if self.top > self.tokens.len() - 1 {
            return &self.tokens[self.tokens.len() - 1];
        }
        &self.tokens[self.top]
    }

    pub fn current_content(&self) -> String {
        self.current().content.clone()
    }

    pub fn current_type(&self) -> TokenType {
        self.current().token_type.clone()
    }

    pub fn position(&self) -> TokenPosition {
        self.current().position.clone()
    }
    
    pub fn expect_type(&self, token: TokenType) -> ResResult<()> {
        if self.current().token_type == token {
            Ok(())
        } else {
            Err(make_error(
                Some(self.current().position),
                format!("expecting type '{:?}', found '{:?}'", token, self.current_content())
            ))
        }
    }

    pub fn consume_type(&mut self, token: TokenType) -> ResResult<String> {
        if self.current().token_type == token {
            let content = self.current_content();
            self.next()?;
            Ok(content)
        } else {
            Err(make_error(
                Some(self.current().position),
                format!("expecting type '{:?}', found '{:?}'", token, self.current_content())
            ))
        }
    }

    pub fn expect_content(&self, content: &str) -> ResResult<()> {
        if self.current_content() == content {
            Ok(())
        } else {
            Err(make_error(
                Some(self.current().position),
                format!("expecting '{}', found '{}'", content, self.current_content())
            ))
        }
    }

    pub fn consume_content(&mut self, content: &str) -> ResResult<String> {
        if self.current().content == content {
            let content = self.current_content();
            self.next()?;
            Ok(content)
        } else {
            Err(make_error(
                Some(self.current().position),
                format!("expecting '{}', found '{}'", content, self.current_content())
            ))
        }
    }
}
