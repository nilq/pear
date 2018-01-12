use super::*;

pub struct Compiler<'c> {
    pub ast: &'c Vec<Statement>
}

impl<'c> Compiler<'c> {
    pub fn new(ast: &'c Vec<Statement>) -> Self {
        Compiler {
            ast,
        }
    }

    pub fn compile(&self) -> ResResult<String> {
        let mut compiled = String::new();

        for statement in self.ast.iter() {
            compiled.push_str(&format!("{}\n", self.compile_statement(&statement)?))
        }

        Ok(compiled)
    }

    fn compile_statement(&self, statement: &Statement) -> ResResult<String> {
        use StatementNode::*;

        match (&statement.0, statement.1) {
            (&Expression(ref expr), _)                       => self.compile_expression(expr),
            (&Definition {ref kind, ref left, ref right}, _) => self.compile_definition(left, right),
            _                                                => Ok(String::new())
        }
    }
    
    fn compile_expression(&self, expression: &Expression) -> ResResult<String> {
        use ExpressionNode::*;

        let c = match expression.0 {
            Number(ref n)     => format!("{}", n),
            Str(ref n)        => format!(r#""{}""#, n),
            Bool(ref n)       => format!("{}", n),
            Identifier(ref n) => format!("{}", n),

            Binary {ref left, ref op, ref right} => self.compile_operation(left, op, right)?,

            _ => String::new(),
        };

        Ok(c)
    }
    
    fn compile_operation(&self, left: &Expression, op: &Operator, right: &Expression) -> ResResult<String> {
        use Operator::*;
        use ExpressionNode::*;
        
        let c = match *op {
            PipeRight => {
                let compiled_left  = self.compile_expression(left)?;
                let compiled_right = self.compile_expression(right)?;

                format!("{}({})", compiled_left, compiled_right)
            },
            
            PipeLeft => {
                let compiled_left  = self.compile_expression(left)?;
                let compiled_right = self.compile_expression(right)?;

                format!("{}({})", compiled_right, compiled_left)
            },
            
            _ => {
                let compiled_left  = self.compile_expression(left)?;
                let compiled_op    = self.compile_operator(op)?;
                let compiled_right = self.compile_expression(right)?;
                
                match right.0 {
                    Number(_) |
                    Str(_)    |
                    Bool(_)   |
                    Identifier(_) => format!("{}{}{}", compiled_left, compiled_op, compiled_right),
                    _             => format!("{}{}({})", compiled_left, compiled_op, compiled_right),
                }
            }
        };

        Ok(c)
    }
    
    fn compile_operator(&self, op: &Operator) -> ResResult<String> {
        use Operator::*;
        
        let c = match *op {
            Add     => "+",
            Sub     => "-",
            Mul     => "*",
            Div     => "/",
            Mod     => "%",
            Pow     => "^",
            Equal   => "==",
            NEqual  => "~=",
            Lt      => "<",
            LtEqual => "<=",
            Gt      => ">",
            GtEqual => ">=",
            Concat  => "..",
            _       => "",
        };
        
        Ok(c.to_owned())
    }
    
    fn compile_definition(&self, left: &Expression, right: &Option<Expression>) -> ResResult<String> {
        let compiled_left  = self.compile_expression(&left)?;

        let mut compiled = format!("local {} ", compiled_left);

        if let Some(ref right) = *right {
            compiled.push_str(&format!("= {}", self.compile_expression(&right)?))
        }

        Ok(compiled)
    }
}
