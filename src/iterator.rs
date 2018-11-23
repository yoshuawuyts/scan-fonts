use std::path::{Path, PathBuf};
use std::iter::Iterator;

/// Iterate over each font.
#[derive(Debug, Clone)]
pub struct FontIterator {
  path: PathBuf,
  cursor: usize
}

impl FontIterator {
  /// Create a new instance.
  pub fn new(path: &Path) -> Self {
    Self {
      path: path.to_path_buf(),
      cursor: 0
    }
  }
}

impl Iterator for FontIterator {
  type Item = super::Font;

  fn next(&mut self) -> Option<Self::Item> {
    unimplemented!();
    // let cursor = self.cursor;
    // self.cursor += 1;

    // if cursor >= self.inner.books.len() {
    //   None
    // } else {
    //   Some(&self.inner.books[cursor])
    // }
  }
}
