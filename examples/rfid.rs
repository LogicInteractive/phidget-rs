fn main() {
    let _rfid = phidget_rs::PhidgetRfid::default()
        .on_tag(|tag| println!("RFID tag: {}", tag))
        .on_tag_lost(|tag| println!("RFID tag lost: {}", tag))
        .build()
        .expect("Failed to create fidget");

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
