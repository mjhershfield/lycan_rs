use std::time::Duration;
use ft60x_rs::{Device, Pipe};
use rand::prelude::*;

fn main() {
    // Load bundled D3XX driver
    ft60x_rs::load_bundled_dylib().expect("loading bundled ftd3xx driver");
    let d3xx_detected = ft60x_rs::d3xx_available();
    println!("D3XX driver was {}detected", if d3xx_detected {""} else {"not "});

    // Get all d3xx devices
    let device_list = ft60x_rs::list_devices().expect("listing d3xx devices connected to system");
    println!("D3XX devices connected: {:?}", device_list);        

    // For now, just use the first d3xx device
    let ftdi_info = match device_list.first() {
        Some(device) => device,
        None => {
            println!("No D3XX devices detected!");
            return;
        }
    };

    println!("Serial number: {}", ftdi_info.serial_number().unwrap());
    println!("Device is currently in use: {}", ftdi_info.is_open());

    let ftdi = Device::open_with_index(0).expect("opening first FTDI device");

    println!("FTDI device acquired: {:?}", ftdi);
    println!("USB 3.0 enabled: {}", ftdi.is_usb3().unwrap());

    let num_loopbacks = 10000;
    let mut successes = 0;
    let mut current_address = 0;
    let mut read_buf = [0u8; 4];


    for _ in 0..num_loopbacks {
        let mut packet: u32 = random();
        packet = packet & (u32::MAX >> 3) | (current_address << 29);


        let write_buf = packet.to_le_bytes();
        match ftdi.write(Pipe::Out0, &write_buf, Duration::from_secs(3)) {
            Ok(res) => println!("Wrote {} bytes: {:X?}", res, write_buf),
            Err(e) => {panic!("Error writing to Out0: {}", e)}
        }

        match ftdi.read(Pipe::In0, read_buf.as_mut_slice(), Duration::from_secs(3)) {
            Ok(res) => println!("Read {} bytes: {:X?}", res, read_buf),
            Err(e) => panic!("Error reading from In0: {}", e)
        };

        let read_packet = u32::from_le_bytes(read_buf);
        if packet == read_packet {
            println!("written {:X} == read {:X}!", packet, read_packet);
            successes += 1;
        }

        current_address = (current_address + 1) % 8;

    }

    println!("Successful Loopbacks: {}/{} ({}%)", successes, num_loopbacks, successes*100/num_loopbacks);

}
