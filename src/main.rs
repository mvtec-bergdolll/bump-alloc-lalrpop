#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub grammar); // synthesized by LALRPOP

mod ast;
mod lex;

use bumpalo::Bump;

use crate::grammar::LangParser;
use crate::lex::Lexer;

fn parse<'input, 'alloc>(input: &'input str, bump: &'alloc Bump) {
    let lexer = Lexer { input };
    let mut errors = Vec::new();

    let parse_result = LangParser::new().parse(&bump, &mut errors, lexer);
    dbg!(parse_result);
}

fn main() {
    let bump = Bump::new();
    let code = String::from("xx");

    parse(&code, &bump);
    println!("Hello, world!");
}
