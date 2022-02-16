use embedded_hal::digital::v2::OutputPin;
use nrf52840_hal::{
    gpio::{Level, Output, Pin, PushPull},
    pac::TIMER0,
    prelude::*,
    timer::OneShot,
    Timer,
};
pub struct LEDColor {
    pub r: Pin<Output<PushPull>>,
    pub b: Pin<Output<PushPull>>,
    pub g: Pin<Output<PushPull>>,
}
impl LEDColor {
    pub fn init<Mode>(led_red: Pin<Mode>, led_blue: Pin<Mode>, led_green: Pin<Mode>) -> LEDColor {
        LEDColor {
            r: led_red.into_push_pull_output(Level::High),
            b: led_blue.into_push_pull_output(Level::High),
            g: led_green.into_push_pull_output(Level::High),
        }
    }

    pub fn red(&mut self) {
        self.r.set_low().unwrap();
        self.g.set_high().unwrap();
        self.b.set_high().unwrap();
    }

    pub fn blue(&mut self) {
        self.r.set_high().unwrap();
        self.g.set_high().unwrap();
        self.b.set_low().unwrap();
    }
    pub fn green(&mut self) {
        self.r.set_high().unwrap();
        self.g.set_low().unwrap();
        self.b.set_high().unwrap();
    }
    pub fn yellow(&mut self) {
        self.r.set_low().unwrap();
        self.g.set_low().unwrap();
        self.b.set_high().unwrap();
    }
    pub fn off(&mut self) {
        self.r.set_high().unwrap();
        self.g.set_high().unwrap();
        self.b.set_high().unwrap();
    }

    pub fn error_blink_red(&mut self, timer: &mut Timer<TIMER0, OneShot>) {
        for _i in 0..10 {
            self.red();
            timer.delay_ms(200_u32);
            self.off();
            timer.delay_ms(200_u32);
        }
    }
}
