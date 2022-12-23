# serialport-hci

[bluetooth-hci][] `Controller` implementation for using a Bluetooth controller
with HCI connected via a serial UART (via [serialport-rs][]).

To use this library you'll need a Bluetooth 4.1 or later controller connected
via a serial UART which accepts HCI commands.

This library relies on exclusive, low-level access to a Bluetooth controller,
and *completely bypasses* your computer's normal Bluetooth stack (ie: bluez,
CoreBluetooth, etc.). This *doesn't* give you raw radio access though – you're
still limited to the commands in the Bluetooth HCI specification.

**This won't work with USB Bluetooth dongles, Bluetooth + WiFi controllers, or
your PC's built-in Bluetooth radio.**

If you want a high-level and easy-to-use Rust library for Bluetooth Low Energy
which works with your PC's normal Bluetooth stack, check out [btleplug][]
(cross-platform) or [bluer][] (Linux-only, but supports advertising).

Like [bluetooth-hci][], this *only* supports low-level control commands. But one
could write a higher-level interface (with support for things like pairing and
GATT) on top of `bluetooth-hci`'s API, and use it with this as a `Controller`.

## Examples

Apache Mynewt has a [HCI controller example][nimble] which can be installed on a
compatible microcontroller with a BLE radio and serial UART. The examples below
have been tested with Mynewt running on a third-party Nordic nRF52832
development board (YJ-16002) with hardware flow control *disabled*.

These examples take one or two command-line arguments:

* (required) the serial port to connect to (eg: `/dev/ttyACM0`, `/dev/ttyUSB0`,
  `/dev/tty.usbserial-XXX`, `COM3`)

* (optional) the baud rate to connect at (default: `1000000`, which is what
  Mynewt uses)

These examples always disable hardware flow control, and assume you've done the
same in your Mynewt build.

These don't rely on any vendor-specific commands.

### BLE Advertising with Eddystone

`./examples/eddystone.rs` advertises an Eddystone beacon for
`https://www.rust-lang.org/`

```
cargo run --example eddystone -- /dev/tty.usbserial-XXXX
```

You can use a mobile app like [nRF Connect for mobile][nrf-connect] to pick this
up.

### Stop BLE advertising

`./examples/stop_advertising.rs` stops advertising:

```
cargo run --example stop_advertising -- /dev/tty.usbserial-XXXX
```

[bluetooth-hci]: https://github.com/danielgallagher0/bluetooth-hci
[bluer]: https://crates.io/crates/bluer
[btleplug]: https://github.com/deviceplug/btleplug
[nimble]: https://mynewt.apache.org/latest/tutorials/ble/blehci_project.html
[nrf-connect]: https://www.nordicsemi.com/Products/Development-tools/nRF-Connect-for-mobile
[serialport-rs]: https://github.com/serialport/serialport-rs
