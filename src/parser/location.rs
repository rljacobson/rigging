/*!

Locations for source code. A location can be transparently attached to a type using `Located<T>`.

There are various types of locations:

 * `Span` defines a range of positions given by the parser,
 * `Documented` attaches doc-comments to locations,
 * `Generated` is used to signify that code is generated based on code from some other location,
 * `Unique` is used internally to tag locations with unique integers so they can be referred to later,
 * `Unknown` is used for Sail that has no obvious corresponding location, although this should be avoided as much
   possible as it leads to poor error messages.

Ast nodes programmatically generated initially have `Unknown` locations, but `Ast_util.locate` can be used to
recursively attach `Generated` locations for error purposes.

*/

use std::ops::{Deref, DerefMut};

use codemap::Span;

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub enum SourceLocation {
  #[default]
  /// The default "empty" location
  Unknown,
  /// Internal tags allowing locations to be referred to later
  Unique(i32, Box<SourceLocation>),
  /// Code is generated based on code from some other location
  Generated(Box<SourceLocation>),
  /// Attaches doc-comments to locations
  Hint(String, Box<SourceLocation>, Box<SourceLocation>),
  /// Location span in the source text
  Span(Span),
}


/// A location can be transparently attached to a type using `Located<T>`.
#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct Located<T> {
  pub location : SourceLocation,
  pub value    : T,
}

impl<T> From<(T, SourceLocation)> for Located<T> {
  fn from((value, location): (T, SourceLocation)) -> Self {
    Located { location, value }
  }
}

impl<T> From<T> for Located<T> {
  fn from(value: T) -> Self {
    Located {
      location: SourceLocation::default(),
      value,
    }
  }
}

impl<T> PartialEq<T> for Located<T>
where
    T: PartialEq,
{
  fn eq(&self, other: &T) -> bool {
    self.value == *other
  }
}

impl<T> Deref for Located<T> {
  type Target = T;

  fn deref(&self) -> &T {
    &self.value
  }
}

impl<T> DerefMut for Located<T> {
  fn deref_mut(&mut self) -> &mut T {
    &mut self.value
  }
}

impl<T, U> AsRef<U> for Located<T>
where
    T: AsRef<U>,
    U: ?Sized,
{
  fn as_ref(&self) -> &U {
    self.value.as_ref()
  }
}

impl<T> std::hash::Hash for Located<T>
where
    T: std::hash::Hash,
{
  fn hash<H>(&self, state: &mut H)
  where
      H: std::hash::Hasher,
  {
    self.location.hash(state);
    self.value.hash(state);
  }
}

impl<T> Located<T> {
  pub fn map<U, F>(self, mut f: F) -> Located<U>
  where
      F: FnMut(T) -> U,
  {
    Located {
      location: self.location,
      value: f(self.value),
    }
  }
}
