use ft60x_rs::{Device, Pipe};
use std::{thread::sleep, time::Duration};

fn main() {
    // Load bundled D3XX driver
    ft60x_rs::load_bundled_dylib().expect("loading bundled ftd3xx driver");
    let d3xx_detected = ft60x_rs::d3xx_available();
    println!(
        "D3XX driver was {}detected",
        if d3xx_detected { "" } else { "not " }
    );

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

    ftdi.set_timeout(Pipe::Out0, Duration::from_millis(3000))
        .unwrap();
    ftdi.set_timeout(Pipe::In0, Duration::from_millis(3000))
        .unwrap();

    let write_buf = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    // Read from input pipe, write to output pipe
    match ftdi.write(Pipe::Out0, &write_buf, Duration::from_secs(3)) {
        Ok(res) => println!("Wrote {} bytes: {:X?}", res, write_buf),
        Err(e) => {
            panic!("Error writing to Out0: {}", e)
        }
    }

    // BUFSTS APPEAR TO HAPPEN IN 16 BYTE BLOCKS! Do we need a hardware revision to ensure we check be for incoming FIFO data?
    // I am thinking that data always gets entered into FIFO, we have to decide whether to read it into our system.
    let write_buf = [
        17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32,
    ];
    match ftdi.write(Pipe::Out0, &write_buf, Duration::from_secs(3)) {
        Ok(res) => println!("Wrote {} bytes: {:X?}", res, write_buf),
        Err(e) => {
            panic!("Error writing to Out0: {}", e)
        }
    }

    sleep(Duration::from_secs(1));

    loop {
        let mut read_buf = vec![0u8; 128];
        match ftdi.read(Pipe::In0, read_buf.as_mut_slice(), Duration::from_secs(3)) {
            Ok(res) => {
                if res == 0 {
                    println!("Read 0 bytes, returning!");
                    return;
                }
                println!("Read {} bytes: {:X?}", res, read_buf);
            }
            Err(e) => panic!("Error reading from In0: {}", e),
        };
    }
}
