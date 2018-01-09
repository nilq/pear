extern crate colored;

mod pear;
use pear::lexer::*;

fn main() {
    let source = r#"
fib: fun (a: number) -> number {
    match a {
        | 0 => 0
        | 1 => 1
        | n => (fib n - 1) + (fib n - 2)
    }
}
    "#;

    let lexer = make_lexer(source.clone().chars().collect());

    for token in lexer {
        println!("{:#?}", token);
    }
}
