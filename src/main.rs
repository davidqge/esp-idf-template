use embedded_storage::nor_flash::{
    ErrorType, NorFlash, NorFlashError, NorFlashErrorKind, ReadNorFlash,
};
use esp_storage::FlashStorage;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");

    let mut bytes = [0u8; 32];

    let mut flash = FlashStorage::new();

    let flash_addr = 0x9000;
    println!("Flash size = {}", flash.capacity());
    println!();

    flash.read(flash_addr, &mut bytes).unwrap();
    println!("Read from {:x}:  {:02x?}", flash_addr, &bytes[..32]);

    bytes[0x00] = bytes[0x00].wrapping_add(1);
    bytes[0x01] = bytes[0x01].wrapping_add(2);
    bytes[0x02] = bytes[0x02].wrapping_add(3);
    bytes[0x03] = bytes[0x03].wrapping_add(4);
    bytes[0x04] = bytes[0x04].wrapping_add(1);
    bytes[0x05] = bytes[0x05].wrapping_add(2);
    bytes[0x06] = bytes[0x06].wrapping_add(3);
    bytes[0x07] = bytes[0x07].wrapping_add(4);

    flash.write(flash_addr, &bytes).unwrap();
    println!("Written to {:x}: {:02x?}", flash_addr, &bytes[..32]);

    let mut reread_bytes = [0u8; 32];
    flash.read(flash_addr, &mut reread_bytes).unwrap();
    println!("Read from {:x}:  {:02x?}", flash_addr, &reread_bytes[..32]);

    println!("Reset (CTRL-R in espflash) to re-read the persisted data.");

    loop {}
}
