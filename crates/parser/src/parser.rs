use std::borrow::Cow;

pub type Loc = usize;
pub type Span = (Loc, Loc);
pub type LexStr<'input> = (&'input str, Span);

pub enum AST<'input> {
    Error,
    Comment(LexStr<'input>),
    Blocks(Vec<Block<'input>>),
}

pub struct Block<'input> {
    header: BlockType,
    name: Cow<'input, str>,
    expr: Expr<'input>,
}

pub enum BlockType {
    Title,
    SubTitle,
    Section,
    Scene,
}

pub struct Title<'input> {
    pub name: &'input str,
}

pub struct SubTitle<'input> {
    pub name: &'input str,
}
pub struct Section<'input> {
    pub name: &'input str,
}

pub struct Scene<'input> {
    pub name: &'input str,
}

pub struct Keyword<'input> {
    pub name: &'input str,
}

pub enum Expr<'input> {
    Error,
}
