//! `serialport-hci` demo for stopping advertising.
use std::{time::Duration, env};

use bluetooth_hci::{
    host::{uart::{CommandHeader, Hci as _}, Hci as _},
};

use serialport::FlowControl;
use serialport_hci::{vendor::none::*, SerialController};

const DEFAULT_BAUD_RATE: u32 = 1_000_000;

fn main() {
    let args: Vec<String> = env::args().collect();
    let port = match args.get(1) {
        None => {
            println!("Usage: {} <port> [baud rate]", args[0]);
            println!("Example: {} /dev/ttyUSB0", args[0]);
            println!("If not specified, baud rate defaults to {}.", DEFAULT_BAUD_RATE);
            panic!("No port selected");
        },
        Some(port) => port,
    };
    let baud_rate = match args.get(2) {
        None => DEFAULT_BAUD_RATE,
        Some(baud_rate) => u32::from_str_radix(baud_rate, 10).expect("Expected baud rate to be an integer"),
    };

    let port = serialport::new(port, baud_rate)
        .timeout(Duration::from_secs(2))
        .flow_control(FlowControl::None)
        .open()
        .expect("Error opening serial port");
    let mut hci: SerialController<CommandHeader, Vendor> = SerialController::new(port);

    println!("Resetting controller");
    hci.reset().unwrap();
    let mut r: bluetooth_hci::host::uart::Packet<Event> = hci.read().unwrap();
    println!("<= {:?}", r);

    println!("Disabling advertising");
    hci.le_set_advertise_enable(false).unwrap();
    r = hci.read().unwrap();
    println!("<= {:?}", r);
}
