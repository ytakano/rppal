// Copyright (c) 2017-2021 Rene van der Meer
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

use embedded_hal::i2c::blocking::{Read, Write, WriteRead};

use super::{Error, I2c};

/// `Write` trait implementation for `embedded-hal` v1.0.0-alpha.5.
impl Write for I2c {
    type Error = Error;

    fn write(&mut self, address: u8, bytes: &[u8]) -> Result<(), Self::Error> {
        self.set_slave_address(u16::from(address))?;
        I2c::write(self, bytes)?;

        Ok(())
    }
}

/// `Write` trait implementation for `embedded-hal` v0.2.6.
impl embedded_hal_0::blocking::i2c::Write for I2c {
    type Error = Error;

    fn write(&mut self, address: u8, bytes: &[u8]) -> Result<(), Self::Error> {
        Write::write(self, address, bytes)
    }
}

/// `Read` trait implementation for `embedded-hal` v1.0.0-alpha.5.
impl Read for I2c {
    type Error = Error;

    fn read(&mut self, address: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
        self.set_slave_address(u16::from(address))?;
        I2c::read(self, buffer)?;

        Ok(())
    }
}

/// `Read` trait implementation for `embedded-hal` v0.2.6.
impl embedded_hal_0::blocking::i2c::Read for I2c {
    type Error = Error;

    fn read(&mut self, address: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
        Read::read(self, address, buffer)
    }
}

/// `WriteRead` trait implementation for `embedded-hal` v1.0.0-alpha.5.
impl WriteRead for I2c {
    type Error = Error;

    fn write_read(
        &mut self,
        address: u8,
        bytes: &[u8],
        buffer: &mut [u8],
    ) -> Result<(), Self::Error> {
        self.set_slave_address(u16::from(address))?;
        I2c::write_read(self, bytes, buffer)?;

        Ok(())
    }
}

/// `WriteRead` trait implementation for `embedded-hal` v0.2.6.
impl embedded_hal_0::blocking::i2c::WriteRead for I2c {
    type Error = Error;

    fn write_read(
        &mut self,
        address: u8,
        bytes: &[u8],
        buffer: &mut [u8],
    ) -> Result<(), Self::Error> {
        WriteRead::write_read(self, address, bytes, buffer)
    }
}
