# framed-serial - Add frames to serial connections. Useful for embedded devices. Can be built with `no_std`. [![Version][version-img]][version-url] [![Status][status-img]][status-url] [![Doc][doc-img]][doc-url]

[version-img]: https://img.shields.io/crates/v/framed-serial.svg
[version-url]: https://crates.io/crates/framed-serial
[status-img]: https://travis-ci.org/astraw/framed-serial.svg?branch=master
[status-url]: https://travis-ci.org/astraw/framed-serial
[doc-img]: https://docs.rs/framed-serial/badge.svg
[doc-url]: https://docs.rs/framed-serial/

See the [documentation](https://docs.rs/framed-serial/).

## Potential improvements

- [ ] use a more elaborate algorithm, such as [COBS](https://crates.io/crates/cobs)
- [ ] detect and recover from errors in the data received, e.g. with checksums
- [ ] base async code on [futures-rs](https://github.com/alexcrichton/futures-rs)

## Running the tests

If you have a device connected sending frames with FramedConnection, execute
tests with:

    cargo test --no-default-features --features device_connected -- --nocapture

## License

Licensed under either of

* Apache License, Version 2.0,
  (./LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license (./LICENSE-MIT or http://opensource.org/licenses/MIT)
  at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

## Code of conduct

Anyone who interacts with framed-serial in any space including but not limited to
this GitHub repository is expected to follow our
[code of conduct](https://github.com/astraw/framed-serial/blob/master/code_of_conduct.md)
