use serial::SerialPort;
use super::embedded_serial;
use super::std;

/// Implment the traits required for a `FramedConnection` based
/// on a `serial::SerialPort`.
///
/// Note that the async processing of `FramedConnection` depends on
/// a short timeout being set. At the time of writing, `serial::open()` returned
/// a SerialPort with a default of 100 msec.
pub struct SerialWrap<T>
    where T: SerialPort,
{
    inner: T,
}

impl<T> SerialWrap<T>
    where T: SerialPort,
{
    /// Constructor
    pub fn new(port: T) -> SerialWrap<T> {
        // do not block
        SerialWrap {inner: port}
    }
}

impl<T> embedded_serial::NonBlockingRx for SerialWrap<T>
    where T: SerialPort,
{
    type Error=();
    fn getc_try(&mut self) -> Result<Option<u8>, Self::Error> {
        let mut buf: [u8;1] = [0];

        match self.inner.read(&mut buf) {
            Ok(1) => Ok(Some(buf[0])),
            Ok(n_bytes) => panic!("no error, but {} bytes read.", n_bytes),
            Err(e) => {
                match e.kind() {
                    std::io::ErrorKind::TimedOut => {Ok(None)},
                    _ => panic!("Can't read, err {:?}", e),
                }
            },
        }
    }
}

impl<T> embedded_serial::NonBlockingTx for SerialWrap<T>
    where T: SerialPort,
{
    type Error=();
    fn putc_try(&mut self, ch: u8) -> Result<Option<()>, Self::Error> {
        let buf: [u8; 1] = [ch];
        loop {
            match self.inner.write(&buf) {
                Ok(1) => {break;},
                Ok(n_bytes) => {panic!("no error, but {} bytes written", n_bytes);},
                Err(e) => {println!("write error {:?}",e); return Err(());}
            }
        }
        Ok(Some(()))
    }
}
