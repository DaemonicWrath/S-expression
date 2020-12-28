mod eval;
mod grammar;
mod lexer;
mod parser;
mod syntax;

use eval::eval;
use lexer::Lexer;
use parser::Parser;
use structopt::StructOpt;

// Using Struct Opt to build our cli parser
#[derive(StructOpt, Debug)]
#[structopt(name = "Calculator")]
struct Opt {
    #[structopt(name = "input")]
    input: String,
}

fn main() {
    let opt = Opt::from_args();
    let mut tokens: Vec<_> = Lexer::new(&opt.input).collect();

    // we need to reverse the tokens as they are poped from the string
    tokens.reverse();

    let parse = Parser::new(tokens).parse();

    if !parse.errors.is_empty() {
        for error in &parse.errors {
            println!("{}", error);
        }
        return;
    }

    let result = eval(parse);
    println!("{}", result);
}
