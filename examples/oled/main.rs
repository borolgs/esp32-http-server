use core::fmt::Write;
use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::i2c::*;
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::hal::prelude::*;
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();

    let peripherals = Peripherals::take()?;
    let i2c = peripherals.i2c0;
    let sda = peripherals.pins.gpio21;
    let scl = peripherals.pins.gpio22;

    let config = I2cConfig::new().baudrate(100.kHz().into());
    let i2c = I2cDriver::new(i2c, sda, scl, &config)?;

    let interface = I2CDisplayInterface::new(i2c);

    let mut display =
        Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0).into_terminal_mode();

    display.init().unwrap();
    display.clear().unwrap();

    loop {
        write!(display, "Test {}", "oled\n")?;

        FreeRtos::delay_ms(500);
        for c in "Hello, ".chars() {
            display.write_char(c)?;
            FreeRtos::delay_ms(100);
        }
        display.write_str("World!")?;
        FreeRtos::delay_ms(500);
        display.clear().unwrap();
    }
}
