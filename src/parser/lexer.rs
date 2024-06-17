use crate::parser::errors::LocatedParseError;
use crate::parser::SpannedToken;

pub type LexerResult<'input>  = Result<SpannedToken<'input>, LocatedParseError>;
