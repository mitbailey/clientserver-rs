use std::io::Write;
use std::net::TcpListener;
use core::str;
use std::thread;
use std::io::prelude::*;
use refimage::{DynamicImageData, GenericImage};
use std::time::SystemTime;
use image::open;

// Uses generic_camera to create a test image that will be serialized and sent to a client on demand.
fn generate_test_image() -> String {
    let img = open("./res/Grass.png").expect("Could not load image");
    let img = DynamicImageData::try_from(img).expect("Could not convert image");

    let mut img = GenericImage::new(SystemTime::now(), img); // Convert to a GenericImage

    // Insert the camera information as metadata.
    let _ = img.insert_key("CAMERA", ("Rust Test Program", "Name of the camera used to capture the image"));
    
    let json = serde_json::to_string(&img).unwrap(); // serialize the image to JSON
    json
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:50042").unwrap();
    println!("listening started, ready to accept");
    for stream in listener.incoming() {
        thread::spawn(|| {
            let mut stream = stream.unwrap();
            let mut buffer = [0; 128];

            println!("New client!");
            stream.write_all(b"Hello World\r\n").unwrap();

            loop {
                if let Err(e) = stream.read(&mut buffer) {
                    eprintln!("Failed to read from stream: {}", e);
                }

                let rx_str = str::from_utf8(&buffer).unwrap();
                if rx_str.starts_with("CMD 0") {
                    println!("Zero (0) command received. Doing 0!");
                    stream.write_all(b"Zero (0) command received. Doing 0!\r\n").unwrap();
                }
                else if rx_str.starts_with("CMD 1") {
                    println!("One (1) command received. Doing 1!");
                    stream.write_all(b"One (1) command received. Doing 1!\r\n").unwrap();
                }
                else if rx_str.starts_with("SEND IMAGE TEST") {
                    println!("Sending image test.");

                    // Serialize and send...
                    stream.write_all(generate_test_image().as_bytes()).unwrap();
                }
                else if rx_str.starts_with("END COMMS") {
                    println!("Ending communication with client.");
                    stream.write_all(b"Ending communication with client.\r\n").unwrap();
                    break;
                }
                else {
                    println!("Unknown command received: {:?}; expected {:?}.", buffer[0], b'1');
                    stream.write_all(b"Unknown command received.\r\n").unwrap();
                }
            }
        });
    }
}