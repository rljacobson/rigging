use codemap::Spanned;

mod ast;
mod lexer;
mod errors;
mod location;


pub type SpannedToken<'input> = Spanned<Token<'input>>;
