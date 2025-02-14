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

    let text_to_write = "Hello world! :)";
    let mut write_buf = [0x48, 0x49, 0x0a, 0x0c];
    let mut buf_ind: usize = 0;
    for character in text_to_write.as_bytes() {
        write_buf[buf_ind] = *character;
        buf_ind += 1;
        if buf_ind == 3 {
            buf_ind = 0;

            // Read from input pipe, write to output pipe
            match ftdi.write(Pipe::Out0, &write_buf, Duration::from_secs(3)) {
                Ok(res) => println!("Wrote {} bytes: {:X?}", res, write_buf),
                Err(e) => {
                    panic!("Error writing to Out0: {}", e)
                }
            }
        }
    }

    // Read data
    println!("Reading...");
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
