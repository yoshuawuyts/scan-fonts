use failure::Error;
use scan_fonts::scan_fonts;

fn main() -> Result<(), Error> {
  for font in scan_fonts("../../rust-lang/wubwub")? {
    println!("font {:?}", font);
  }
  Ok(())
}
