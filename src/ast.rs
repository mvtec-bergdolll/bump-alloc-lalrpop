#[derive(Debug)]
pub enum Expr<'input, 'alloc> {
    A(&'alloc Expr<'input, 'alloc>),
    B(&'alloc Expr<'input, 'alloc>),
    End(&'input str),
}
