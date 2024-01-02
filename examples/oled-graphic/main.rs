use embedded_graphics::{
    mono_font::{ascii::FONT_4X6, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Line, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle},
    text::{Baseline, Text},
};
use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::i2c::*;
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::hal::prelude::*;
use qrcodegen::{QrCode, QrCodeEcc, QrSegment, Version};
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};
use std::ops::Add;

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();

    let peripherals = Peripherals::take()?;
    let i2c = peripherals.i2c0;
    let sda = peripherals.pins.gpio21;
    let scl = peripherals.pins.gpio22;

    let config = I2cConfig::new().baudrate(100.kHz().into());
    let i2c = I2cDriver::new(i2c, sda, scl, &config)?;

    let interface = I2CDisplayInterface::new(i2c);

    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    display.init().unwrap();

    // QR Code

    let segs = QrSegment::make_segments("hello!");
    let qr = QrCode::encode_segments_advanced(
        &segs,
        QrCodeEcc::Medium,
        Version::new(2),
        Version::new(2),
        None,
        true,
    )
    .unwrap();

    draw_qrcode(&mut display, &qr, &Point::zero(), None).unwrap();

    Line::new(Point::new(0, 32), Point::new(128, 32))
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
        .draw(&mut display)
        .unwrap();

    // Text

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_4X6)
        .text_color(BinaryColor::On)
        .build();

    let mut txt_display = display.cropped(&Rectangle::new(Point::new(32, 0), Size::new(96, 32)));

    Text::with_baseline("Hello world!", Point::zero(), text_style, Baseline::Top)
        .draw(&mut txt_display)
        .unwrap();

    Text::with_baseline(
        "This text will be cropped",
        Point::new(0, 16),
        text_style,
        Baseline::Top,
    )
    .draw(&mut txt_display)
    .unwrap();

    display.flush().unwrap();

    loop {
        FreeRtos::delay_ms(500);
    }
}

fn draw_qrcode<D>(
    display: &mut D,
    qr: &QrCode,
    point: &Point,
    module_size: Option<u32>,
) -> Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor>,
{
    let size = module_size.unwrap_or(1);
    let rect_size = Size::new(size, size);

    for y in 0..qr.size() {
        for x in 0..qr.size() {
            if qr.get_module(x, y) {
                let pt = point.add(Point::new(x * size as i32, y * size as i32));

                Rectangle::new(pt, rect_size)
                    .into_styled(
                        PrimitiveStyleBuilder::new()
                            .fill_color(BinaryColor::On)
                            .build(),
                    )
                    .draw(display)?;
            }
        }
    }

    Ok(())
}
