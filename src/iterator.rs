use crate::font;
use failure::Error;
use std::convert::TryFrom;
use std::fs::{self, DirEntry};
use std::io;
use std::iter::Iterator;
use std::path::{Path, PathBuf};

/// Iterate over each font.
#[derive(Debug)]
pub struct FontIterator {
  path: PathBuf,
  cursor: usize,
  entries: Vec<DirEntry>,
}

impl FontIterator {
  /// Create a new instance.
  pub fn open(dir: &Path) -> Result<Self, Error> {
    let mut entries = vec![];
    visit_dirs(dir, &mut entries)?;
    Ok(Self {
      entries,
      path: dir.to_path_buf(),
      cursor: 0,
    })
  }
}

impl Iterator for FontIterator {
  type Item = Result<font::Font, Error>;

  #[inline]
  fn next(&mut self) -> Option<Self::Item> {
    let entry = match self.entries.get(self.cursor) {
      Some(entry) => entry,
      None => return None,
    };

    let entry = font::Font::try_from(entry);
    self.cursor += 1;
    Some(entry)
  }
}

fn visit_dirs(dir: &Path, entries: &mut Vec<DirEntry>) -> io::Result<()> {
  for entry in fs::read_dir(dir)? {
    let entry = entry?;
    let path = entry.path();
    if path.is_dir() {
      visit_dirs(&path, entries)?;
    } else if is_font(&path) {
      entries.push(entry);
    }
  }
  Ok(())
}

/// Check if a path on the directory points to a font file.
#[inline]
fn is_font(file: &Path) -> bool {
  let ext = match file.extension() {
    Some(str) => str,
    None => return false,
  };

  let ext = match ext.to_str() {
    Some(str) => str,
    None => return false,
  };

  match ext {
    "ttf" => true,
    "otf" => true,
    _ => false,
  }
}
