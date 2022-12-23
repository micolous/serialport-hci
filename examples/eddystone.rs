//! `serialport-hci` demo for transmitting an Eddystone beacon for
//! <https://www.rust-lang.org/>
//! 
//! To stop advertising, see `stop_advertising.rs`.
use std::{time::Duration, env};

use bluetooth_hci::{
    host::{uart::{CommandHeader, Hci as _}, AdvertisingParameters, OwnAddressType, Channels, Hci as _},
    types::{Advertisement, AdvertisingInterval, AdvertisingType}, BdAddr,
};
use rand::{thread_rng, Rng};
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

    
    let mut rng = thread_rng();
    let mut addr = [0u8; 6];
    addr[5] = 0xc0;
    rng.fill(&mut addr[..5]);

    // URL: https://www.rust-lang.org/
    let eddystone_data = b"\x10\x20\x01rust-lang\x01";

    // Encode into service data
    let mut service_data = [0; 24];
    let adv = Advertisement::ServiceData16BitUuid(0xfeaa, eddystone_data);
    let len = adv.copy_into_slice(&mut service_data);

    let p = AdvertisingParameters {
        advertising_interval: AdvertisingInterval::for_type(
            AdvertisingType::NonConnectableUndirected,
        )
        .with_range(Duration::from_millis(100), Duration::from_millis(500))
        .unwrap(),
        own_address_type: OwnAddressType::Random,
        peer_address: bluetooth_hci::BdAddrType::Random(bluetooth_hci::BdAddr([0xc0; 6])),
        advertising_channel_map: Channels::all(),
        advertising_filter_policy:
            bluetooth_hci::host::AdvertisingFilterPolicy::WhiteListConnectionAllowScan,
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

    println!("Setting a random address: {:02x?}", addr);
    hci.le_set_random_address(BdAddr(addr))
        .unwrap();
    r = hci.read().unwrap();
    println!("<= {:?}", r);

    println!("Setting advertising parameters: {:?}", p);
    hci.le_set_advertising_parameters(&p).unwrap();
    r = hci.read().unwrap();
    println!("<= {:?}", r);

    println!("Setting advertising data: {:02x?}", &service_data[..len]);
    hci.le_set_advertising_data(&service_data[..len]).unwrap();
    r = hci.read().unwrap();
    println!("<= {:?}", r);

    println!("Enabling advertising");
    hci.le_set_advertise_enable(true).unwrap();
    r = hci.read().unwrap();
    println!("<= {:?}", r);

    println!("Open a beacon scanner on your mobile device, you should see an Eddystone beacon for https://www.rust-lang.org/");
}
