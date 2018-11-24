/// Font style
#[derive(Debug, Clone)]
pub enum FontStyle {
  /// Default font style.
  Normal,
  /// Thick.
  Bold,
  /// Slanted.
  Italic,
  /// Crossed out.
  Oblique,
}

impl Default for FontStyle {
  fn default() -> Self {
    FontStyle::Normal
  }
}
