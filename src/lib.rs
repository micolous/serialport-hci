//! Implements a wrapper around `serialport` for a Bluetooth HCI connected via a
//! serial UART.
pub mod vendor;

use bluetooth_hci::{Controller, Vendor as HciVendor};
use core::marker::PhantomData;
use serialport::SerialPort;
use std::io::{Error, ErrorKind};

const BUFFER_SIZE: usize = 6;

/// Wrapper for serial BLE controller
pub struct SerialController<Header, Vendor>
where
    Vendor: HciVendor,
{
    /// The underlying [SerialPort] to connect to.
    port: Box<dyn SerialPort>,

    /// A small buffer used for [Self.peek].
    buffer: [u8; BUFFER_SIZE],
    buffer_len: usize,
    _header: PhantomData<Header>,
    _vendor: PhantomData<Vendor>,
}

impl<Header, Vendor: HciVendor> SerialController<Header, Vendor> {
    /// Attaches a controller to a serial port
    pub fn new(port: Box<dyn SerialPort>) -> Self {
        Self {
            port,
            buffer: [0; BUFFER_SIZE],
            buffer_len: 0,
            _header: PhantomData,
            _vendor: PhantomData,
        }
    }
}

impl<Header, Vendor: HciVendor> Controller for SerialController<Header, Vendor> {
    type Error = Error;
    type Header = Header;
    type Vendor = Vendor;

    fn write(&mut self, header: &[u8], payload: &[u8]) -> nb::Result<(), Self::Error> {
        // println!("write({:02x?}, {:02x?})", header, payload);
        (&mut self.port).write(header)?;
        (&mut self.port).write(payload)?;
        (&mut self.port).flush()?;
        Ok(())
    }
    fn read_into(&mut self, mut buffer: &mut [u8]) -> nb::Result<(), Self::Error> {
        // println!("read_into({})", buffer.len());
        // we may have bytes in buffer to return first
        if self.buffer_len > 0 {
            let l = usize::min(self.buffer_len, buffer.len());
            buffer[..l].copy_from_slice(&self.buffer[..l]);
            // println!(" copied {} bytes from buffer: {:02x?}", l, &self.buffer[..l]);
            buffer = &mut buffer[l..];
            self.buffer_len -= l;

            if self.buffer_len > 0 {
                // didn't read all the bytes from buffer, shift it across
                // println!(" incomplete buffer read");
                self.buffer.copy_within(l..self.buffer_len, 0);
                // self.buffer.rotate_left(l);
                return Ok(());
            }
        }

        if buffer.len() > 0 {
            // println!(" need to read {} more bytes", buffer.len());
            (&mut self.port).read(buffer)?;
            // println!(" extra read: {:02x?}", buffer);
        }

        Ok(())
    }

    fn peek(&mut self, n: usize) -> nb::Result<u8, Self::Error> {
        // println!("peek({})", n);
        // println!(" bytes_to_read = {}", self.port.bytes_to_read().unwrap());
        if n >= BUFFER_SIZE {
            return Err(nb::Error::Other(ErrorKind::InvalidInput.into()));
        }
        if n >= self.buffer_len {
            // println!("trying to read {} bytes", (n + 1) - self.buffer_len);
            self.buffer_len += (&mut self.port).read(&mut self.buffer[self.buffer_len..n + 1])?;
        }

        if n < self.buffer_len {
            // println!(" => {}, {}", self.buffer_len, self.buffer[n]);
            Ok(self.buffer[n])
        } else {
            Err(nb::Error::Other(ErrorKind::WouldBlock.into()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
