#![no_main]
#![no_std]

use core::panic;

use co2 as _;
use co2::buzzer;
// global logger + panicking-behavior + memory layout
// access to board peripherals:
use co2::co2_mod;
use co2::rgb_led;
use co2::scd30;

use nrf52840_hal::{
    self as hal,
    gpio::{p0::Parts as P0Parts, Level},
    prelude::*,
    twim::{self, Twim},
    Timer,
};

#[cortex_m_rt::entry]
fn main() -> ! {
    let board = hal::pac::Peripherals::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let pins = P0Parts::new(board.P0);

    //Buzzer
    let mut buzzer = buzzer::Buzzer::init(pins.p0_28.degrade());
    buzzer.buzz(&mut timer, 440, 500);

    //LED
    //LED on board
    let mut led_1 = pins.p0_13.into_push_pull_output(Level::Low);
    //RGB led
    let led_channel_red = pins.p0_29.degrade();
    let led_channel_blue = pins.p0_30.degrade();
    let led_channel_green = pins.p0_31.degrade();
    let mut light = rgb_led::LEDColor::init(led_channel_red, led_channel_blue, led_channel_green);

    // instanciate I2C
    let scl = pins.p0_03.into_floating_input().degrade();
    let sda = pins.p0_04.into_floating_input().degrade();

    let pins_i2c = twim::Pins { scl, sda };
    let i2c = Twim::new(board.TWIM0, pins_i2c, twim::Frequency::K100);
    let mut sensor = scd30::SCD30::init(i2c);

    let firmware_version = sensor.get_firmware_version().unwrap_or_else(|error| {
        light.error_blink_red(&mut timer);
        panic!("Error getting firmware version: {:?}", error)
    });

    defmt::info!(
        "Firmware Version: {:?}.{:?}",
        firmware_version[0],
        firmware_version[1]
    );

    let pressure = 994_u16; //air pressure in copenhagen
    sensor.start_continuous_measurement(pressure).unwrap();

    loop {
        if sensor.data_ready().unwrap() {
            defmt::info!("Data ready.");
            light.green();
            break;
        } else {
            light.red();
        }
    }

    loop {
        let result = sensor.read_measurement().unwrap_or_else(|error| {
            light.error_blink_red(&mut timer);
            panic!("Error getting data: {:?}", error)
        });

        let co2 = result.co2;
        let temp = result.temperature;
        let humidity = result.humidity;

        defmt::info!(
            "
            CO2 {=f32} ppm
            Temperature {=f32} °C
            Humidity {=f32} %
            ",
            co2,
            temp,
            humidity
        );

        co2_mod::check_levels(&co2, &mut light);

        timer.delay_ms(2000_u32);
        led_1.set_high().unwrap();
        timer.delay_ms(2000_u32);
        led_1.set_low().unwrap();
    }
}
