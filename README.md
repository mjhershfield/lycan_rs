# lycan.rs

right now its just a bunch of main files. I copy the file I want to run to be main.rs and then run `cargo run`

Current files:
- `write_2` makes two writes to the lycan and reads them back in smaller chunks
- `many_loopback` makes a large number of small writes, immediately reading back after each write.
- `uart` encodes a string in our packet format and sends it to the Lycan's peripheral 0. It then reads from Lycan until it reaches a timeout.
