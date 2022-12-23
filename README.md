# serialport-hci

[bluetooth-hci][] Controller implementation for using a Bluetooth HCI controller
connected via a serial UART.

This uses [serialport-rs][] to communicate with the controller.

Apache Mynewt has a [HCI controller example][nimble] which can be installed on a
compatible microcontroller with a BLE radio and serial UART.

See `./examples/eddystone.rs` for an example which configures advertising for
an Eddystone beacon.

[bluetooth-hci]: https://github.com/danielgallagher0/bluetooth-hci
[serialport-rs]: https://github.com/serialport/serialport-rs
[nimble]: https://mynewt.apache.org/latest/tutorials/ble/blehci_project.html
