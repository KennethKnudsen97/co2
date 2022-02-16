#![no_main]
#![no_std]

use co2 as _; // global logger + panicking-behavior + memory layout
use nb::block;
use nrf52840_hal::{self as hal, gpio::p0::Parts as P0Parts, prelude::*, Temp, Timer};

use co2::dk_button;
use co2::rgb_led;

enum Unit {
    Fahrenheit,
    Celsius,
    Kelvin,
}

impl Unit {
    fn convert_temperature(&self, temperature: f32) -> f32 {
        match self {
            Unit::Fahrenheit => (temperature * 1.8) + 32.0,

            Unit::Kelvin => temperature + 273.15,

            Unit::Celsius => temperature,
        };
        temperature
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    let board = hal::pac::Peripherals::take().unwrap();
    let pins = P0Parts::new(board.P0);

    //Buttom
    let mut button_1 = dk_button::Button::new(pins.p0_11.degrade());

    //Timer
    let mut periodic_timer = Timer::periodic(board.TIMER0);
    let mut millis: u64 = 0;

    //TEMP
    let mut temp = Temp::new(board.TEMP);
    let mut current_unit = Unit::Celsius;

    //LED
    let led_channel_red = pins.p0_03.degrade();
    let led_channel_blue = pins.p0_04.degrade();
    let led_channel_green = pins.p0_28.degrade();
    let mut light = rgb_led::LEDColor::init(led_channel_red, led_channel_blue, led_channel_green);

    loop {
        periodic_timer.start(1000u32);

        if (millis % 1000) == 0 {
            defmt::info!("Tick (milliseconds): {=u64}", millis);
            let temperature: f32 = temp.measure().to_num();
            let converted_temp = current_unit.convert_temperature(temperature);
            match current_unit {
                Unit::Fahrenheit => defmt::println!("{=f32} °F", converted_temp),
                Unit::Kelvin => defmt::println!("{=f32} K", converted_temp),
                Unit::Celsius => defmt::println!("{=f32} °C", converted_temp),
            };

            if temperature > 24.0 {
                light.red();
            } else if temperature < 23.0 {
                light.blue();
            } else {
                light.green();
            }
        };

        if (millis % 5) == 0 && button_1.check_rising_edge() {
            current_unit = match current_unit {
                Unit::Fahrenheit => Unit::Kelvin,
                Unit::Kelvin => Unit::Celsius,
                Unit::Celsius => Unit::Fahrenheit,
            };
            defmt::println!("Unit changed");
        };

        block!(periodic_timer.wait()).unwrap();
        millis = millis.saturating_add(1);
    }
}
