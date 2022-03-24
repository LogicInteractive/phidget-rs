use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let running_ctrlc = running.clone();

    ctrlc::set_handler(move || running_ctrlc.store(false, Ordering::SeqCst))
        .expect("Error setting Ctrl-C handler");

    phidget_rs::PhidgetRfid::default()
        .on_tag(|tag| println!("RFID tag: {}", tag))
        .on_tag_lost(|tag| println!("RFID tag lost: {}", tag))
        .run(move |phidget_rfid| {
            println!("RFID attached: {:?}", phidget_rfid.handle);
            while running.load(Ordering::SeqCst) {
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        })
        .expect("Failed to create PhidgetRfid");
}
