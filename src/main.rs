use embedded_storage::{self, ReadStorage};
use esp_storage::FlashStorage;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();
    // let peripherals = Peripherals::take().unwrap();
    let flash = FlashStorage::new();
    log::info!("Flsh capacity: {}", flash.capacity());
}
