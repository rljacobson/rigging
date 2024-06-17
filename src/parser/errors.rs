#![allow(dead_code)]
/*!

This module defines errors related to scanning and parsing.

*/

use std::{error::Error, fmt::Display};
use std::fmt::{Debug, Formatter};

use crate::parser::location::Located;

pub type LocatedParseError   = Located<ParserError>;
// pub type LalrpopError<'input> = lalrpop_util::ParseError<ByteIndex, Token<'input>, SpannedParserError>;
// pub type Errors<'input>       = SmallVec<LalrpopError<'input>>;

#[derive(Clone, Eq, PartialEq)]
pub enum ParserError {
  UnterminatedStringLiteral,
  // ParseIntegerError(ParseIntegerError),
  // ParseRealNumberError(ParseRealError),
  MalformedNumberLiteral(char), // Invalid character in number literal
  UnrecognizedCharacter(char),  // Character outside the alphabet of the language (non-operator, not alphanumeric...)
  UnmatchedOpenBlock,
  UnmatchedCloseBlock,
  UnknownOperator,
  // UnknownError(Box<dyn Error>),
}

impl ParserError {
  pub fn is_fatal(&self) -> bool {
    match self {

      // ParserError::ParseIntegerError(_)
      // | ParserError::ParseRealNumberError(_)
      | ParserError::MalformedNumberLiteral(_)
      | ParserError::UnmatchedOpenBlock
      | ParserError::UnmatchedCloseBlock
      | ParserError::UnknownOperator => false,

      // | ParserError::UnknownError(_)
      | ParserError::UnrecognizedCharacter(_)
      | ParserError::UnterminatedStringLiteral => true,

    }
  }

  pub fn msg(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      ParserError::UnterminatedStringLiteral => {
        write!(f, "unterminated string literal")
      }

      // ParserError::ParseIntegerError(kind) => {
      //   write!(f, "failed to parse integer: {}", kind)
      // }
      //
      // ParserError::ParseRealNumberError(kind) => {
      //   write!(f, "failed to parse real number: {}", kind)
      // }

      ParserError::MalformedNumberLiteral(n) => {
        write!(f, "number literal is this base cannot contain {}", n)
      }

      ParserError::UnrecognizedCharacter(c) => {
        write!(f, "unrecognized character {}", c)
      }

      ParserError::UnmatchedOpenBlock => {
        write!(f, "unmatched `{{`")
      }

      ParserError::UnmatchedCloseBlock => {
        write!(f, "unmatched `}}`")
      }

      ParserError::UnknownOperator => {
        write!(f, "unknown operator")
      }

      // ParserError::UnknownError(_) => {
      //   write!(f, "unknown error")
      // }
    }
  }
}

impl Debug for ParserError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    self.msg(f)
  }
}

impl Display for ParserError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    self.msg(f)
  }
}

impl Error for ParserError {}
