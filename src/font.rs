use crate::font_style::FontStyle;
use failure::{bail, Error};
use getset::Getters;
use std::convert::TryFrom;
use std::fs::DirEntry;
use std::path::PathBuf;

/// A font file on the filesystem.
#[derive(Debug, Getters, Default)]
pub struct Font {
  /// Access the name of the font.
  #[get = "pub"]
  name: String,
  /// Access the path of the font file on the filesystem.
  #[get = "pub"]
  path: PathBuf,
  /// Access the font weight property.
  #[get = "pub"]
  weight: Option<u16>,
  /// Access the font style property.
  #[get = "pub"]
  style: crate::font_style::FontStyle,
}

impl TryFrom<DirEntry> for Font {
  type Error = Error;

  fn try_from(entry: DirEntry) -> Result<Self, Self::Error> {
    let name = match entry.file_name().into_string() {
      Ok(string) => string,
      Err(_) => bail!("Could not convert OsString to String"),
    };

    Ok(Font {
      name,
      path: entry.path(),
      style: FontStyle::default(),
      weight: None,
    })
  }
}

impl TryFrom<&DirEntry> for Font {
  type Error = Error;

  fn try_from(entry: &DirEntry) -> Result<Self, Self::Error> {
    let name = match entry.file_name().into_string() {
      Ok(string) => string,
      Err(_) => bail!("Could not convert OsString to String"),
    };

    Ok(Font {
      name,
      path: entry.path(),
      style: FontStyle::default(),
      weight: None,
    })
  }
}
