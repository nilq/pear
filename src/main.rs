extern crate colored;

mod pear;
use pear::lexer::*;
use pear::parser::*;
use pear::visitor::*;

fn main() {
    let source =
r#"10 + 10  + 1 * "hey""#;

    let path = "source.pear";

    let lines = source.lines().map(|x| x.to_string()).collect();
    let lexer = make_lexer(source.clone().chars().collect(), &lines, &path);

    let mut parser = Parser::new(lexer.collect());

    match parser.parse() {
        Err(response) => response.display(&lines, path),
        Ok(mut ast)   => {
            println!("{:#?}", ast);
            
            let visitor = Visitor::new(&mut ast);

            match visitor.validate() {
                Err(response) => response.display(&lines, path),
                Ok(_)         => (),
            }
        },
    }
}
