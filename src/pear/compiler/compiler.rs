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
            (&Expression(ref expr), _) => self.compile_expression(expr),
        }
    }
    
    fn compile_expression(&self, expression: &Expression) -> ResResult<String> {
        use ExpressionNode::*;

        let c = match expression.0 {
            Number(ref n)     => format!("{}", n),
            Str(ref n)        => format!(r#""{}""#, n),
            Bool(ref n)       => format!("{}", n),
            Identifier(ref n) => format!("{}", n),

            Binary {ref left, ref op, ref right} => {
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

            _ => String::new(),
        };

        Ok(c)
    }
    
    fn compile_operator(&self, op: &Operator) -> ResResult<String> {
        use Operator::*;
        
        let c = match *op {
            Add => "+",
            Sub => "-",
            Mul => "*",
            Div => "/",
            Mod => "%",
            Pow => "^",
            Equal  => "==",
            NEqual => "~=",
            Lt => "<",
            LtEqual => "<=",
            Gt => ">",
            GtEqual => ">=",
            Concat  => "..",

            _ => "",
        };
        
        Ok(c.to_owned())
    }
}
