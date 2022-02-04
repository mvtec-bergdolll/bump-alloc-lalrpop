#[derive(Clone, Debug)]
pub enum Token<'input> {
    A(&'input str),
    B(&'input str),
    C(&'input str),
}

pub struct Lexer<'input> {
    pub input: &'input str,
}

pub type LexError = u8;
pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<Token<'input>, usize, LexError>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
