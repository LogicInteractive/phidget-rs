use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let running_ctrlc = running.clone();
    let running_a = running.clone();
    let running_b = running.clone();

    ctrlc::set_handler(move || running_ctrlc.store(false, Ordering::SeqCst))
        .expect("Error setting Ctrl-C handler");

    let t1 = std::thread::spawn(move || {
        phidget_rs::PhidgetRfid::default()
            .on_tag(|tag| println!("RFID tag: {}", tag))
            .on_tag_lost(|tag| println!("RFID tag lost: {}", tag))
            .run(move |phidget_rfid| {
                println!("RFID attached: {:?}", phidget_rfid.handle);
                while running_a.load(Ordering::SeqCst) {
                    std::thread::sleep(std::time::Duration::from_secs(1));
                }
            })
            .expect("Failed to create PhidgetRfid");
    });

    let t2 = std::thread::spawn(move || {
        phidget_rs::PhidgetRfid::default()
            .on_tag(|tag| println!("RFID tag: {}", tag))
            .on_tag_lost(|tag| println!("RFID tag lost: {}", tag))
            .run(move |phidget_rfid| {
                println!("RFID attached: {:?}", phidget_rfid.handle);
                while running_b.load(Ordering::SeqCst) {
                    std::thread::sleep(std::time::Duration::from_secs(1));
                }
            })
            .expect("Failed to create PhidgetRfid");
    });

    let _ = t1.join();
    let _ = t2.join();
}
