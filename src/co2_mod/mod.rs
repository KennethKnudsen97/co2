use crate::rgb_led::LEDColor;

const UPPER_LIMIT: f32 = 2000.0;
const WARN_LIMIT: f32 = 1000.0;

pub fn check_levels(co2: &f32, led: &mut LEDColor) {
    if *co2 < WARN_LIMIT {
        led.green();
    } else if *co2 > WARN_LIMIT && *co2 < UPPER_LIMIT {
        led.yellow();
    } else {
        led.red();
    }
}
