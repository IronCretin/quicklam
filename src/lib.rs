use std::rc::Rc;

use lalrpop_util::lexer::Token;
use lalrpop_util::{lalrpop_mod, ParseError};

pub use term::Term;

pub mod error;
pub mod reduce;
pub mod term;

lalrpop_mod!(parser);

pub fn parse(src: &str) -> Result<Rc<Term<'_>>, ParseError<usize, Token<'_>, error::Error>> {
    parser::TermParser::new().parse(&mut vec![], src)
}
