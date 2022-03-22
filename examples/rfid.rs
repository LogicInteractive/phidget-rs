fn main() {
    phidget_rs::PhidgetRfid::default()
        .on_tag(|tag| println!("RFID tag: {}", tag))
        .on_tag_lost(|tag| println!("RFID tag lost: {}", tag))
        .run(|phidget_rfid| {
            println!("RFID attached: {:?}", phidget_rfid.handle);
            loop {
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        })
        .expect("Failed to create PhidgetRfid");
}
