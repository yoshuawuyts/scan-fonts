#![forbid(unsafe_code, future_incompatible)]
#![forbid(rust_2018_idioms, rust_2018_compatibility)]
#![deny(missing_debug_implementations, bad_style)]
#![deny(missing_docs)]
#![cfg_attr(test, deny(warnings))]

//! ## Example
//! ```rust
//! use scan_fonts::scan_fonts;
//!
//! for font in scan_fonts(".") {
//!   println!("font {}", font.name);
//! }
//! ```

use std::{path::Path, convert::AsRef};

mod font;
mod iterator;

pub use self::iterator::FontIterator;
pub use self::font::Font;

/// Recursively find all fonts in the path.
pub fn scan_fonts (path: impl AsRef<Path>) -> FontIterator{
  let path = path.as_ref();
  FontIterator::new(path)
}
