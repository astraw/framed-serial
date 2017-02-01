# Change Log

All user visible changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/), as described
for Rust libraries in [RFC #1105](https://github.com/rust-lang/rfcs/blob/master/text/1105-api-evolution.md)

## unreleased

### Changed

* Update to `embedded-serial` version 0.4.

* Preliminary support for starting within an ongoing transfer. The
  initial implementation starts each frame with a sentinel value. Future
  work could specify an encoding that ensures the sentinel is not used
  elsewhere.
