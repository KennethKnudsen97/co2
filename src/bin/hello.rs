#![no_main]
#![no_std]

use co2 as _; // global logger + panicking-behavior + memory layout
use embedded_hal::digital::v2::OutputPin;
use nrf52840_hal::{
    self as hal,
    gpio::{p0::Parts as P0Parts, Input, Level, Output, Pin, PullUp, PushPull},
    prelude::*,
};

struct LEDColor {
    r: Pin<Output<PushPull>>,
    b: Pin<Output<PushPull>>,
    g: Pin<Output<PushPull>>,
}
impl LEDColor {
    pub fn init<Mode>(led_red: Pin<Mode>, led_blue: Pin<Mode>, led_green: Pin<Mode>) -> LEDColor {
        LEDColor {
            r: led_red.into_push_pull_output(Level::High),
            b: led_blue.into_push_pull_output(Level::High),
            g: led_green.into_push_pull_output(Level::High),
        }
    }

    fn red(&mut self) {
        self.r.set_low().unwrap();
        self.g.set_high().unwrap();
        self.b.set_high().unwrap();
    }

    fn blue(&mut self) {
        self.r.set_high().unwrap();
        self.g.set_high().unwrap();
        self.b.set_low().unwrap();
    }
    fn green(&mut self) {
        self.r.set_high().unwrap();
        self.g.set_low().unwrap();
        self.b.set_high().unwrap();
    }
    fn off(&mut self) {
        self.r.set_high().unwrap();
        self.g.set_high().unwrap();
        self.b.set_high().unwrap();
    }
}

pub struct Button(Pin<Input<PullUp>>);

impl Button {
    fn new<Mode>(pin: Pin<Mode>) -> Self {
        Button(pin.into_pullup_input())
    }
    pub fn is_pressed(&self) -> bool {
        self.0.is_low().unwrap()
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Hello, world!");

    let board = hal::pac::Peripherals::take().unwrap();

    let pins = P0Parts::new(board.P0);

    let led_channel_red = pins.p0_03.degrade();
    let led_channel_blue = pins.p0_04.degrade();
    let led_channel_green = pins.p0_28.degrade();
    let button_1 = Button::new(pins.p0_11.degrade());

    let mut light = LEDColor::init(led_channel_red, led_channel_blue, led_channel_green);
    let mut led_state = 0;

    loop {
        if button_1.is_pressed() {
            led_state += 1;
        }

        if led_state >= 3 {
            led_state = 0;
        }

        match led_state {
            0 => light.red(),
            1 => light.blue(),
            2 => light.green(),
            _ => light.off(),
        }
    }
}
