use nrf52840_hal::{
    gpio::{Level, Output, Pin, PushPull},
    pac::TIMER0,
    prelude::*,
    timer::OneShot,
    Timer,
};

pub struct Buzzer {
    pin: Pin<Output<PushPull>>,
}

impl Buzzer {
    pub fn init<Mode>(pin: Pin<Mode>) -> Self {
        Buzzer {
            pin: pin.into_push_pull_output(Level::Low),
        }
    }
    fn high(&mut self) {
        self.pin.set_high().unwrap();
    }

    fn low(&mut self) {
        self.pin.set_low().unwrap();
    }

    pub fn buzz(
        &mut self,
        timer: &mut Timer<TIMER0, OneShot>,
        frequency_hz: u32,
        duration_ms: u32,
    ) {
        let delay_ms = frequency_to_delay(&frequency_hz);
        let max_range = duration_to_range(duration_ms, &frequency_hz);

        for _i in 0..max_range {
            self.high();
            timer.delay_ms(delay_ms);
            self.low();
            timer.delay_ms(delay_ms);
        }
    }
}

fn frequency_to_delay(frequency_hz: &u32) -> u32 {
    1000 / frequency_hz
}

fn duration_to_range(duration_ms: u32, frequency_hz: &u32) -> i32 {
    let delay = frequency_to_delay(frequency_hz);
    let max_range = duration_ms / (delay * 2_u32);
    max_range as i32
}
