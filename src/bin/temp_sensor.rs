#![no_main]
#![no_std]

use co2 as _; // global logger + panicking-behavior + memory layout
use embedded_hal::blocking::delay::DelayMs;

use nrf52840_hal::{self as hal, Temp, Timer};

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Hello, world!");

    let board = hal::pac::Peripherals::take().unwrap();

    let mut timer = Timer::new(board.TIMER0);

    let mut temp = Temp::new(board.TEMP);

    loop {
        let temperature: f32 = temp.measure().to_num();
        defmt::println!("{:?} °C", temperature);

        timer.delay_ms(1000_u32)
    }
}
