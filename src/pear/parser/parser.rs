use super::lexer::*;
use super::*;

use std::rc::Rc;

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
        
        if expression.0 == ExpressionNode::EOF {
            Ok(expression)
        } else {
            let backup_top = self.top;
            
            self.skip_types(vec![TokenType::Whitespace])?;

            if self.current_type() == TokenType::Operator {
                self.binary(expression)
            } else {
                self.top = backup_top;

                Ok(expression)
            }
        }
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

    fn binary(&mut self, expression: Expression) -> ResResult<Expression> {
        let mut ex_stack = vec![expression];
        let mut op_stack: Vec<(Operator, u8)> = Vec::new();

        op_stack.push(Operator::from(&self.current_content()).unwrap());
        self.next()?;

        ex_stack.push(self.atom()?);

        let mut done = false;

        while ex_stack.len() > 1 {
            if !done {
                self.skip_types(vec![TokenType::Whitespace])?;

                if self.current_type() != TokenType::Operator {
                    println!("{:#?}", ex_stack);
                    done = true;
                    continue
                }

                let (op, precedence) = Operator::from(&self.current_content()).unwrap();
                self.next()?;

                if precedence >= op_stack.last().unwrap().1 {
                    let left  = ex_stack.pop().unwrap();
                    let right = ex_stack.pop().unwrap();

                    ex_stack.push(
                        Expression::new(
                            ExpressionNode::Binary {
                                right: Rc::new(left),
                                op:    op_stack.pop().unwrap().0,
                                left:  Rc::new(right),
                            },
                            self.position(),
                        )
                    );

                    let term = self.atom()?;

                    ex_stack.push(term);
                    op_stack.push((op, precedence));

                    continue
                }
                
                let term = self.atom()?;

                ex_stack.push(term);
                op_stack.push((op, precedence));
            }

            let left  = ex_stack.pop().unwrap();
            let right = ex_stack.pop().unwrap();

            ex_stack.push(
                Expression::new(
                    ExpressionNode::Binary {
                        right: Rc::new(left),
                        op:    op_stack.pop().unwrap().0,
                        left:  Rc::new(right),
                    },
                    self.position(),
                )
            );
        }

        Ok(ex_stack.pop().unwrap())
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
