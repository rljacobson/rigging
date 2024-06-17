/*!

A thin abstraction layer over external dependencies.

It is convenient to have an abstraction layer to provide a stable interface to functionality the
implementation of which can be swapped out. For example, we might start with using the string
interner for LALRPOP (`lalrpop-intern`). If we later change out minds, it is easy to use a different
backing library so long as we use a stable API implemented here.

This is also a natural place to put shims for any case of an option to use an external library for
some built-in functionality.

*/

// Define the type alias BigInteger based on whether the bigint feature is enabled
#[cfg(feature = "bigint")]
use num_bigint::BigInt;
#[cfg(feature = "bigint")]
pub type BigInteger = BigInt;

#[cfg(not(feature = "bigint"))]
pub type BigInteger = i64;

use string_cache::{DefaultAtom};

pub type IString    = DefaultAtom;

/// Interns a `String`
#[inline(always)]
pub fn interned_string(text: String) -> IString {
  IString::from(text)
}

/// Interns a `&str`.
#[inline(always)]
pub fn interned<S>(text: S) -> IString
    where S: Into<String>
{
  IString::from(text.into())
}

#[inline(always)]
pub fn interned_static(text: &'static str) -> IString {
  interned(text)
}

/// Produces a `String` from `atom`.
pub fn resolve(atom: IString) -> String {
  atom.to_string()
}

