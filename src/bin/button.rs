#![no_main]
#![no_std]

use co2 as _; // global logger + panicking-behavior + memory layout
use nb::block;
use nrf52840_hal::{
    self as hal,
    gpio::{p0::Parts as P0Parts, Input, Pin, PullUp},
    prelude::*,
    Temp, Timer,
};

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

pub struct Button {
    pin: Pin<Input<PullUp>>,
    was_pressed: bool,
}

impl Button {
    fn new<Mode>(pin: Pin<Mode>) -> Self {
        Button {
            pin: pin.into_pullup_input(),
            was_pressed: false,
        }
    }
    pub fn is_pressed(&self) -> bool {
        self.pin.is_low().unwrap()
    }

    fn check_rising_edge(&mut self) -> bool {
        let mut rising_edge = false;

        let is_pressed = self.is_pressed();
        // Only trigger on "rising edge" of the signal
        // Term: "Edge Triggering"
        if self.was_pressed && !is_pressed {
            // Was pressed, now isn't:
            rising_edge = true;
        }
        self.was_pressed = is_pressed;
        rising_edge
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    let board = hal::pac::Peripherals::take().unwrap();
    let pins = P0Parts::new(board.P0);

    let mut current_unit = Unit::Celsius;
    let mut button_1 = Button::new(pins.p0_11.degrade());
    let mut periodic_timer = Timer::periodic(board.TIMER0);
    let mut millis: u64 = 0;
    let mut temp = Temp::new(board.TEMP);

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

            // measure temperature
            // display temperature
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
