//! Add frames to serial connections. Useful for embedded devices. Can be built with `no_std`.
//!
//! The main type of interest is [`FramedConnection`](struct.FramedConnection.html), which takes ownership
//! of a serial connection and allows sending and receiving complete frames.
//!
//! To use with the standard library, put this in your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! framed-serial = "0.1"
//! ```
//!
//! To use in an embedded device with `no_std`, put this in your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! framed-serial = {version = "0.1", default-features = false, features = ["collections"]}
//! ```
//!
//! Example usage:
//!
//! ```
//! extern crate serial;
//! extern crate framed_serial;
//! use serial::SerialPort;
//!
//! fn wait_for_frame() {
//!
//!     let mut raw = serial::open("/dev/ttyACM0").expect("open serial port");
//!
//!     // Async processing depends on this being short.
//!     raw.set_timeout(std::time::Duration::from_millis(100)).expect("set_timeout");
//!
//!     let my_ser = framed_serial::SerialWrap::new(raw);
//!     let mut conn = framed_serial::FramedConnection::new(my_ser);
//!
//!     // Loop until we get a frame. This requires a connected device
//!     // sending with FramedConnection.
//!     loop {
//!         let tick_state = conn.tick();
//!         if tick_state.recv_is_done {
//!             let data = conn.get_frame().expect("get_frame()");
//!             println!("{:?}", data);
//!             break;
//!         }
//!     }
//! }
//!
//! // This example requires std to compile. To run successfully, it further requires a connected
//! // serial device on /dev/ttyACM0 implementing `FramedConnection`. Use conditional compilation
//! // to run only if the `device_connected` feature was specified at compile time.
//!
//! #[cfg(feature = "device_connected")]
//! fn main() {
//!     wait_for_frame();
//! }
//! #[cfg(not(feature = "device_connected"))]
//! fn main() {
//!     // Do nothing if device is not connected.
//! }
//! ```
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "collections", feature(collections))]
#![deny(missing_docs)]
extern crate embedded_serial;
extern crate byteorder;

#[cfg(feature = "collections")]
extern crate collections;

#[cfg(feature = "std")]
extern crate serial;

#[cfg(feature = "std")]
mod core {
    pub use std::mem;
}

use embedded_serial::{NonBlockingTx, NonBlockingRx};
use byteorder::ByteOrder;

#[cfg(feature = "collections")]
use collections::vec::Vec;

#[cfg(feature = "std")]
mod serialwrap;

#[cfg(feature = "std")]
pub use serialwrap::SerialWrap;

struct HeaderState {
    bytes: [u8; 2],
    index: usize,
}

struct DataState {
    length: usize,
}

enum RecvState {
    Header(HeaderState),
    Data(DataState),
}

struct SendingState{
    is_header: bool,
    index: usize,
    header_bytes: [u8; 2],
    frame: Vec<u8>,
}

enum SendState {
    NotSending,
    Sending(SendingState),
}

/// The result of a `tick()`. Check for progress indication.
pub struct TickProgress {
    /// State of ongoing receive.
    pub recv_is_done: bool,
    /// State of ongoing send.
    pub send_is_done: bool,
}

/// Wrapper around a serial port to provide framed connections.
///
/// See the module level documentation for more information.
pub struct FramedConnection<S>
    where S : NonBlockingRx + NonBlockingTx,
{
    serial: S,
    recv_buf: Vec<u8>,
    recv_state: RecvState,
    send_state: SendState,
}

impl<S> FramedConnection<S>
    where S : NonBlockingRx + NonBlockingTx,
{
    /// Create a new `FramedConnection`. Takes ownership of the serial device.
    pub fn new(s:S) -> FramedConnection<S> {
        FramedConnection {
            serial:s,
            recv_buf: Vec::new(),
            recv_state: FramedConnection::<S>::_start_recv_state(),
            send_state: FramedConnection::<S>::_start_send_state(),
            }
    }

    fn _start_recv_state() -> RecvState {
        RecvState::Header( HeaderState{bytes: [0, 0], index: 0})
    }

    fn _start_send_state() -> SendState {
        SendState::NotSending
    }

    /// Schedule a frame to be sent. Returns `Err(())` if the frame is too long,
    /// otherwise returns immediately with `Ok(())`.
    pub fn schedule_send(&mut self, frame: Vec<u8>) -> Result<(),()> {
        if frame.len() > u16::max_value() as usize {
            return Err(());
        }
        let mut buf = [0; 2];
        byteorder::LittleEndian::write_u16(&mut buf, frame.len() as u16);
        self.send_state = SendState::Sending( {
            SendingState{
                is_header: true,
                index: 0,
                header_bytes: buf,
                frame: frame,
            }});
        Ok(())
    }

    /// Service the connection.
    pub fn tick(&mut self) -> TickProgress {
        TickProgress {
            send_is_done: self._send_tick(),
            recv_is_done: self._recv_tick(),
        }
    }

    /// return bool to describe whether send is done.
    fn _send_tick(&mut self) -> bool {
        match self.send_state {
            SendState::NotSending => {
                return true;
            },
            SendState::Sending(ref mut s) => {
                loop {
                    // while we are not blocked on send, keep sending.
                    let byte = match s.is_header {
                        true => s.header_bytes[s.index],
                        false => s.frame[s.index],
                    };
                    match self.serial.putc_try(byte) {
                        Ok(()) => {
                            s.index += 1;
                            if s.is_header {
                                if s.index == 2 {
                                    s.is_header = false;
                                    s.index = 0;
                                }
                            } else {
                                if s.index == s.frame.len() {
                                    // don't send more
                                    break;
                                }
                            }
                        },
                        Err(_) => {
                            // error during send
                            return false;
                        }
                    }
                }
            }
        }
        // we have completed sending a frame
        self.send_state = SendState::NotSending;
        true
    }

    /// return bool to describe whether recv is done.
    fn _recv_tick(&mut self) -> bool {

        loop {
            // While we get characters, keep looping.

            if self.is_frame_complete() {
                return true;
            }

            match self.serial.getc_try() {
                Ok(byte) => {
                    let mut new_state: Option<RecvState> = None;
                    match self.recv_state {
                        RecvState::Header(ref mut hs) => {
                            hs.bytes[hs.index] = byte;
                            hs.index += 1;
                            if hs.index == 2 {
                                let ds = DataState {
                                    length: byteorder::LittleEndian::read_u16(&hs.bytes) as usize,
                                };
                                new_state = Some(RecvState::Data(ds));
                            }
                        },
                        RecvState::Data(ref mut ds) => {
                            self.recv_buf.push(byte);
                            if self.recv_buf.len() == ds.length {
                                // this frame is complete, stop polling for new data
                                return true;
                            }
                        },
                    };
                    if let Some(ns) = new_state {
                        self.recv_state=ns;
                    }
                },
                Err(_) => {
                    // no more data available
                    break;
                },
            };

        }
        false
    }

    /// Check if frame is complete.
    fn is_frame_complete(&mut self) -> bool {
        match self.recv_state {
            RecvState::Header(_) => false,
            RecvState::Data(ref ds) => ds.length == self.recv_buf.len(),
        }
    }

    /// Get completed frame.
    pub fn get_frame(&mut self) -> Result<Vec<u8>,()> {
        let frame = match self.recv_state {
            RecvState::Header(_) => {
                return Err(());
            },
            RecvState::Data(ref ds) => {
                if self.recv_buf.len() == ds.length {
                    let mut frame = Vec::with_capacity(0);
                    core::mem::swap(&mut self.recv_buf,&mut frame);
                    frame
                } else {
                    return Err(());
                }
            },
        };
        self.recv_state = FramedConnection::<S>::_start_recv_state();
        Ok(frame)
    }

}
