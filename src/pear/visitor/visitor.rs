use super::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Number,
    Bool,
    Str,
    Nil,
}

pub struct Visitor<'v> {
    pub ast: &'v Vec<Statement>,
}

impl<'v> Visitor<'v> {
    pub fn new(ast: &'v Vec<Statement>) -> Self {
        Visitor {
            ast,
        }
    }
    
    pub fn validate(&self) -> ResResult<()> {
        for statement in self.ast.iter() {
            self.visit_statement(statement)?
        }
        
        Ok(())
    }
    
    fn visit_statement(&self, statement: &Statement) -> ResResult<()> {
        use StatementNode::*;

        match (&statement.0, statement.1) {
            (&Expression(ref expr), _) => self.visit_expression(expr),
        }
    }
    
    fn visit_expression(&self, expression: &Expression) -> ResResult<()> {
        use ExpressionNode::*;

        match (&expression.0, expression.1) {
            (&Binary { .. }, _) => match self.type_expression(&expression) {
                Ok(_)    => Ok(()),
                Err(err) => Err(err),
            }
            _ => Ok(())
        }
    }

    fn type_expression(&self, expression: &Expression) -> ResResult<Type> {
        use ExpressionNode::*;

        let t = match (&expression.0, expression.1) {
            (&Number(_), _) => Type::Number,
            (&Bool(_), _)   => Type::Bool,
            (&Str(_), _)    => Type::Str,
            (&Binary { ref left, ref op, ref right }, position) => {
                use Operator::*;

                match (self.type_expression(&*left)?, op, self.type_expression(&*right)?) {
                    (a, &Add, b) => match (a, b) {
                        (Type::Number, Type::Number) => Type::Number,
                        (a, b)                       => return Err(make_error(Some(position), format!("can't add {:?} and {:?}", a, b)))
                    },

                    (a, &Sub, b) => match (a, b) {
                        (Type::Number, Type::Number) => Type::Number,
                        (a, b)                       => return Err(make_error(Some(position), format!("can't subtract {:?} and {:?}", a, b)))
                    },

                    (a, &Mul, b) => match (a, b) {
                        (Type::Number, Type::Number) => Type::Number,
                        (a, b)                       => return Err(make_error(Some(position), format!("can't multiply {:?} and {:?}", a, b)))
                    },

                    _ => Type::Nil,
                }
            },
            _ => Type::Nil,
        };

        Ok(t)
    }
}
