use esp_idf_svc::hal::{delay::FreeRtos, gpio::PinDriver, peripherals::Peripherals};

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let mut led = PinDriver::input_output(peripherals.pins.gpio2).unwrap();

    loop {
        led.set_high()?;
        log::info!("{}", led.is_high());
        FreeRtos::delay_ms(1000);

        led.set_low()?;
        log::info!("{:?}", led.is_high());
        FreeRtos::delay_ms(1000);
    }
}
