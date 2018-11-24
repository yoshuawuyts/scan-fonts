#![forbid(unsafe_code, future_incompatible)]
#![forbid(rust_2018_idioms, rust_2018_compatibility)]
#![deny(missing_debug_implementations, bad_style)]
#![deny(missing_docs)]
#![cfg_attr(test, deny(warnings))]
#![feature(try_from)]

//! ## Example
//! ```rust
//! use scan_fonts::scan_fonts;
//!
//! # fn main () -> Result<(), failure::Error> {
//!   for font in scan_fonts(".")? {
//!     println!("font {:?}", font);
//!   }
//! # Ok(())}
//! ```

use failure::{Error, ResultExt};
use std::{convert::AsRef, path::Path};

mod font;
mod font_style;
mod iterator;

pub use self::font::Font;
pub use self::font_style::FontStyle;
pub use self::iterator::FontIterator;

/// Recursively find all fonts in the path.
pub fn scan_fonts(path: impl AsRef<Path>) -> Result<FontIterator, Error> {
  let path = path.as_ref();
  let iter =
    FontIterator::open(path).context("Could not create font iterator")?;
  Ok(iter)
}
