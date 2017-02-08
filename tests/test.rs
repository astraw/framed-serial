extern crate embedded_serial;
extern crate framed_serial;

use framed_serial::FramedConnection;

struct MockSerial {
    in_flight: Vec<u8>,
}

impl MockSerial {
    fn new() -> MockSerial {
        MockSerial { in_flight: Vec::new() }
    }
}

impl embedded_serial::NonBlockingRx for MockSerial {
    type Error=();
    fn getc_try(&mut self) -> Result<Option<u8>, Self::Error> {
        if self.in_flight.len() < 1 {
            return Ok(None);
        }
        Ok( Some(self.in_flight.remove(0)) )
    }
}

impl embedded_serial::NonBlockingTx for MockSerial {
    type Error=();
    fn putc_try(&mut self, ch: u8) -> Result<Option<()>, Self::Error> {
        self.in_flight.push(ch);
        Ok(Some(()))
    }
}

fn test_buffer(original: &[u8]) {
    let mut conn = FramedConnection::new(MockSerial::new());
    conn.schedule_send(original.to_vec()).unwrap();
    loop {
        let tick_state = conn.tick();
        if tick_state.unwrap().recv_is_done {
            let valid = conn.get_frame().unwrap();
            assert!(valid == original);

            // because this is just a test, we break here
            break;
        }
    }
}

#[test]
fn test_1() {
    test_buffer(b"123");
}

#[cfg(feature = "std")]
extern crate serial;

// Always compile this with std, but only run with device_connected.
#[allow(dead_code)]
#[cfg(feature = "std")]
fn wait_for_frame() -> Result<(),framed_serial::Error> {

    let raw = serial::open("/dev/ttyACM0").expect("open serial port");

    let my_ser = framed_serial::SerialWrap::new(raw);
    let mut conn = framed_serial::FramedConnection::new(my_ser);

    println!("looping until we get a frame...");
    // Loop until we get a frame. This requires a connected device sending with FramedConnection.
    loop {
        let tick_state = conn.tick();
        if tick_state?.recv_is_done {
            let data = conn.get_frame()?;
            println!("{:?}", data);
            break;
        }
    }
    Ok(())
}

#[cfg(feature = "device_connected")]
#[test]
fn test_wait_for_frame() {
    wait_for_frame().unwrap();
}
