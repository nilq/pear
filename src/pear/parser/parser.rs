use super::lexer::*;

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

    fn next(&mut self) -> ResResult<()> {
        if self.top < self.tokens.len() {
            self.top += 1;
            Ok(())
        } else {
            Err(make_error(None, "moving nexting outside token stack".to_owned()))
        }
    }

    fn back(&mut self) -> ResResult<()> {
        if self.top > 0 {
            self.top -= 1;
            Ok(())
        } else {
            Err(make_error(None, "moving backing outside token stack".to_owned()))
        }
    }

    fn remaining(&self) -> usize {
        self.tokens.len() - self.top + 1
    }

    pub fn current(&self) -> &Token {
        &self.tokens[self.tokens.len() - 1]
    }
    
    pub fn current_content(&self) -> String {
        self.current().content.clone()
    }
    
    pub fn expect_type(&self, token: TokenType) -> ResResult<()> {
        if self.current().token_type == token {
            Ok(())
        } else {
            Err(make_error(
                Some(ResponseLocation::new(self.current().position, self.current_content().len())),
                format!("expecting type '{:?}', found '{:?}'", token, self.current_content())
            ))
        }
    }

    pub fn consume_type(&mut self, token: TokenType) -> ResResult<String> {
        if self.current().token_type == token {
            self.next()?;
            Ok(self.current_content())
        } else {
            Err(make_error(
                Some(ResponseLocation::new(self.current().position, self.current_content().len())),
                format!("expecting type '{:?}', found '{:?}'", token, self.current_content())
            ))
        }
    }

    pub fn expect_content(&self, content: &str) -> ResResult<()> {
        if self.current_content() == content {
            Ok(())
        } else {
            Err(make_error(
                Some(ResponseLocation::new(self.current().position, self.current_content().len())),
                format!("expecting '{}', found '{}'", content, self.current_content())
            ))
        }
    }

    pub fn consume_content(&mut self, content: &str) -> ResResult<String> {
        if self.current().content == content {
            self.next()?;
            Ok(self.current_content())
        } else {
            Err(make_error(
                Some(ResponseLocation::new(self.current().position, self.current_content().len())),
                format!("expecting '{}', found '{}'", content, self.current_content())
            ))
        }
    }
}
