# Change Log

All user visible changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/), as described
for Rust libraries in [RFC #1105](https://github.com/rust-lang/rfcs/blob/master/text/1105-api-evolution.md)

## [0.4.0] - 2017-05-11

### Changed

* Update to `embedded-serial` version 0.5.

## [0.3.1] - 2017-02-19

### Changed

* Error return when attempting to send while previous send ongoing.

### Added

* Implemented `FramedConnection::block_until_send_done()` to wait for
  previous send to complete.

## [0.3.0] - 2017-02-11

### Changed

* Implemented `framed_serial::Error` and `framed_serial::Result` types.
  Changed the API to use them. Removed all potential panics.
* Tests using serial device use port specified by environment variable
  `DEVICE`.

## [0.2.0] - 2017-02-01

### Changed

* Update to `embedded-serial` version 0.4.

* Preliminary support for starting within an ongoing transfer. The
  initial implementation starts each frame with a sentinel value. Future
  work could specify an encoding that ensures the sentinel is not used
  elsewhere.
