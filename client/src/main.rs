use core::str;
use std::io::prelude::*;
use std::net::TcpStream;

use image::DynamicImage;
use refimage::{FitsCompression, FitsWrite, GenericImageOwned};
use std::path::Path;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:50042")?;
    let mut buffer = [0; 4096];
    
    let _ = stream.read(&mut buffer[..])?;

    println!("Rxed Msg (Exp. Hello): {}", str::from_utf8(&buffer).unwrap());

    stream.write_all(b"CMD 0")?;

    let _ = stream.read(&mut buffer[..])?;

    println!("Rxed Msg (Exp. CMD 0): {}", str::from_utf8(&buffer).unwrap());

    stream.write_all(b"CMD 1")?;

    let _ = stream.read(&mut buffer[..])?;

    println!("Rxed Msg (Exp. CMD 1): {}", str::from_utf8(&buffer).unwrap());

    // Image test transfer.
    stream.write_all(b"SEND IMAGE TEST")?;
    let _ = stream.read(&mut buffer[..])?;
    println!("Rxed Msg (Exp. SEND IMAGE TEST): {}", str::from_utf8(&buffer).unwrap());

    // RX and deserialize...
    let rimg: GenericImageOwned = serde_json::from_str(str::from_utf8(&buffer).unwrap().trim_end_matches(char::from(0))).unwrap(); // Deserialize to generic image.
    println!("{:?}", rimg.get_metadata());
    println!("{:?}", rimg.get_image());
    let img: DynamicImage = rimg.get_image().clone().try_into().expect("Could not convert image");
    img.save("received.png").unwrap();
    // let rimg = rimg.into_luma().expect("Could not convert to luma");
    rimg.write_fits(Path::new("received.fits"), FitsCompression::None, true).unwrap();

    // Complete communications with server.
    stream.write_all(b"END COMMS")?;

    Ok(())
}