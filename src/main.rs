use embedded_storage::{self, ReadStorage};
use esp_storage::FlashStorage;
// use esp_idf_svc::{
//     self,
//     hal::{delay::FreeRtos, gpio::PinDriver, peripherals::Peripherals},
// };

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();
    // let peripherals = Peripherals::take().unwrap();
    let flash = FlashStorage::new();
    log::info!("Flsh capacity: {}", flash.capacity());

    // let button_1 = PinDriver::input(peripherals.pins.gpio32).unwrap();
    // let mut button_2 = PinDriver::input(peripherals.pins.gpio33).unwrap();

    // loop {
    //     if button_1.is_high() {
    //         log::info!("Button 1 pressed on pin {}", button_1.pin());
    //     }
    //     if button_2.is_high() {
    //         log::info!("Button 2 pressed on pin {}", button_2.pin());
    //     }
    //     FreeRtos::delay_ms(20);
    // }
}
