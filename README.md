# scan-fonts
[![crates.io version][1]][2] [![build status][3]][4]
[![downloads][5]][6] [![docs.rs docs][7]][8]

Recursively list which fonts are available in a directory.

- [Documentation][8]
- [Crates.io][2]
- [Releases][releases]

## Examples
__Basic usage__
```rust
use scan_fonts::scan_fonts;

for font in scan_fonts(".") {
  println!("font {}", font.name);
}
```

## Installation
```sh
$ cargo add scan-fonts
```

## Safety
This crate uses ``#![deny(unsafe_code)]`` to ensure everything is implemented in
100% Safe Rust.

## Contributing
Want to join us? Check out our ["Contributing" guide][contributing] and take a
look at some of these issues:

- [Issues labeled "good first issue"][good-first-issue]
- [Issues labeled "help wanted"][help-wanted]

## References
None.

## License
[MIT](./LICENSE-MIT) OR [Apache-2.0](./LICENSE-APACHE)

[1]: https://img.shields.io/crates/v/scan-fonts.svg?style=flat-square
[2]: https://crates.io/crates/scan-fonts
[3]: https://img.shields.io/travis/yoshuawuyts/scan-fonts/master.svg?style=flat-square
[4]: https://travis-ci.org/yoshuawuyts/scan-fonts
[5]: https://img.shields.io/crates/d/scan-fonts.svg?style=flat-square
[6]: https://crates.io/crates/scan-fonts
[7]: https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square
[8]: https://docs.rs/scan-fonts

[releases]: https://github.com/yoshuawuyts/scan-fonts/releases
[contributing]: https://github.com/yoshuawuyts/scan-fonts/blob/master.github/CONTRIBUTING.md
[good-first-issue]: https://github.com/yoshuawuyts/scan-fonts/labels/good%20first%20issue
[help-wanted]: https://github.com/yoshuawuyts/scan-fonts/labels/help%20wanted
