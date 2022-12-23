//! No-op [HciVendor] implementation for `bluetooth-hci`.
//! 
//! Use this as a simple way to interact with a controller without any
//! vendor-specific commands.
use std::convert::TryFrom;

use bluetooth_hci::{
    event::{Error, VendorEvent, VendorReturnParameters},
    BadStatusError,
    Vendor as HciVendor,
};

#[derive(Clone, Debug)]
pub enum ReturnParameters {}
impl VendorReturnParameters for ReturnParameters {
    type Error = ();
    fn new(_bytes: &[u8]) -> Result<Self, Error<Self::Error>> {
        Err(bluetooth_hci::event::Error::Vendor(()))
    }
}

#[derive(Debug, Clone)]
pub enum Status {}

impl TryFrom<u8> for Status {
    type Error = BadStatusError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Err(BadStatusError::BadValue(value))
    }
}

impl From<Status> for u8 {
    fn from(_status: Status) -> Self {
        0
    }
}

#[derive(Debug)]
pub struct Event {}

impl VendorEvent for Event {
    type Error = ();
    type ReturnParameters = ReturnParameters;
    type Status = Status;

    fn new(buffer: &[u8]) -> Result<Self, Error<Self::Error>> {
        Err(Error::UnknownEvent(buffer[0]))
    }
}

#[derive(Debug)]
pub struct Vendor {}

impl HciVendor for Vendor {
    type Status = Status;
    type Event = Event;
}
