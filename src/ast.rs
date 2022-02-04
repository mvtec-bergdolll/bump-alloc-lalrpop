#[derive(Debug)]
pub enum Expr<'input> {
    A(Box<Expr<'input>>),
    B(Box<Expr<'input>>),
    End(&'input str),
}
