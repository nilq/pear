extern crate colored;

mod pear;
use pear::lexer::*;
use pear::parser::*;

fn main() {
    let source =
r#"
100
100.2

"hello world"

hey
hello_yes
_yes_hello

what?
grr!

true
"#;

    let path = "source.pear";

    let lines = source.lines().map(|x| x.to_string()).collect();
    let lexer = make_lexer(source.clone().chars().collect(), &lines, &path);

    let mut parser = Parser::new(lexer.collect());

    match parser.parse() {
        Ok(ast)       => println!("{:#?}", ast),
        Err(response) => response.display(&lines, path),
    }
}
