use std::rc::Rc;
use super::token::{Token, TokenType};
use super::tokenizer::Tokenizer;
use super::matcher::*;

pub fn make_lexer(data: Vec<char>) -> Lexer {
    let tokenizer = Tokenizer::new(data);
    let mut lexer = Lexer::new(tokenizer);

    lexer.matchers_mut().push(Rc::new(NumberLiteralMatcher));
    lexer.matchers_mut().push(Rc::new(StringLiteralMatcher));

    let bool_matcher = ConstantStringMatcher::new(TokenType::Bool, &["true", "false"]);
    lexer.matchers_mut().push(Rc::new(bool_matcher));

    let key_matcher = KeyMatcher::new(TokenType::Keyword, &[
        "fun", "match", "->"
    ]);
    lexer.matchers_mut().push(Rc::new(key_matcher));

    lexer.matchers_mut().push(Rc::new(IdentifierMatcher));

    let eol_matcher = ConstantCharMatcher::new(TokenType::EOL, &['\n']);
    lexer.matchers_mut().push(Rc::new(eol_matcher));

    let operator_matcher = ConstantStringMatcher::new(TokenType::Operator, &[
        "++", "+", "-", "*", "/", "^^", "^", ">=", "<=", "==", "!=", "<|", "|>", "<", ">", "%",
    ]);

    lexer.matchers_mut().push(Rc::new(operator_matcher));

    let symbol_matcher = ConstantCharMatcher::new(TokenType::Symbol, &[
        '(', ')', '[', ']', '{', '}', ',', ':', ';', '!', '|', '=', '\\', '.'
    ]);

    lexer.matchers_mut().push(Rc::new(symbol_matcher));

    lexer.matchers_mut().push(Rc::new(WhitespaceMatcher));
    
    lexer
}

pub struct Lexer {
    tokenizer: Tokenizer,
    matchers: Vec<Rc<Matcher>>,
}

impl Lexer {
    pub fn new(tokenizer: Tokenizer) -> Lexer {
        Lexer {
            tokenizer,
            matchers: Vec::new(),
        }
    }

    pub fn match_token(&mut self) -> Option<Token> {
        for matcher in &mut self.matchers {
            match self.tokenizer.try_match_token(matcher.as_ref()) {
                Some(t) => return Some(t),
                None => continue,
            }
        }
        None
    }

    pub fn matchers(&self) -> &Vec<Rc<Matcher>> {
        &self.matchers
    }

    pub fn matchers_mut(&mut self) -> &mut Vec<Rc<Matcher>> {
        &mut self.matchers
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        let token = match self.match_token() {
            Some(n) => n,
            None    => return None,
        };
        match token.token_type {
            TokenType::EOF => None,
            _ => Some(token),
        }
    }
}
