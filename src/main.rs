use ft60x_rs::{Device, Pipe};
use std::{io, time::Duration};

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
    let stdin = io::stdin();
    let mut text_to_write = String::new();
    stdin.read_line(&mut text_to_write).unwrap();
    text_to_write.pop();
    // for _ in 0..(3 - text_to_write.len() % 3) % 3 {
    //     text_to_write.push('\0');
    // }
    println!(
        "Sending '{}' (padded to length {})",
        text_to_write,
        text_to_write.len()
    );

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

    if buf_ind != 0 {
        write_buf[3] = (buf_ind as u8) << 2;

        // Read from input pipe, write to output pipe
        match ftdi.write(Pipe::Out0, &write_buf, Duration::from_secs(3)) {
            Ok(res) => println!("Wrote {} bytes: {:X?}", res, write_buf),
            Err(e) => {
                panic!("Error writing to Out0: {}", e)
            }
        }
    }

    // Read data
    println!("Reading...");
    let mut zero_counter = 0;
    let mut completed = false;
    let mut full_string = Vec::new();
    while !completed {
        let mut read_buf = [0u8; 64];
        let mut read_amt = 0;
        while read_amt == 0 {
            match ftdi.read(Pipe::In0, read_buf.as_mut_slice(), Duration::from_secs(3)) {
                Ok(res) => {
                    read_amt = res;
                    if read_amt != 0 {
                        println!("Read {} bytes: {:X?}", res, read_buf);
                        zero_counter = 0;
                    } else {
                        zero_counter += 1;
                        if zero_counter > 1000 {
                            completed = true;
                            break;
                        }
                    }
                }
                Err(_) => {
                    completed = true;
                    break;
                }
            };
        }

        // Value has been read into the read_buf
        full_string.extend_from_slice(&read_buf[0..read_amt]);
    }

    // Ghetto packet stripping
    let mut keep_counter = 0;
    full_string.retain(|_| {
        keep_counter = (keep_counter + 1) % 4;
        return keep_counter != 0;
    });

    println!("Read data: ");
    println!("{}", String::from_utf8(full_string).unwrap());
}
