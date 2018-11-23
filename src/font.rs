use std::path::PathBuf;
use getset::Getters;

/// A font file on the filesystem.
#[derive(Debug, Getters)]
pub struct Font {
  /// Access the name of the font.
  #[get = "pub"]
  name: String,
  /// Access the path of the font file on the filesystem.
  #[get = "pub"]
  path: PathBuf,
}
