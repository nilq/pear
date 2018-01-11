extern crate colored;
use colored::Colorize;

mod pear;
use pear::lexer::*;
use pear::parser::*;
use pear::visitor::*;
use pear::compiler::*;

fn main() {
    let source =
r#"10 + 10  + 1 * 3"#;

    let path = "source.pear";

    let lines = source.lines().map(|x| x.to_string()).collect();
    let lexer = make_lexer(source.clone().chars().collect(), &lines, &path);

    let mut parser = Parser::new(lexer.collect());

    match parser.parse() {
        Err(response) => response.display(&lines, path),
        Ok(mut ast)   => {
            println!("{:#?}", ast);
            
            let visitor = Visitor::new(&ast);
            
            
            println!("{} {}", "Checking".green().bold(), path);

            match visitor.validate() {
                Err(response) => response.display(&lines, path),
                Ok(_)         => {

                    let compiler = Compiler::new(&ast);
                    
                    println!("{} {}", "Compiling".green().bold(), path);

                    let compiled = match compiler.compile() {
                        Err(response) => response.display(&lines, path),
                        Ok(a)         => {
                            
                            println!("\n```lua\n{}```", a)
                        }
                    };
                }
            }
        },
    }
}
