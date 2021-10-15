use rust_gpiozero::*;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let led = LED::new(18); // set a variable for the led on pin 18
    loop { // start an infinite loop
        led.on();
        sleep(Duration::from_secs(1)); // pause one second
        led.off();
        sleep(Duration::from_secs(1));
    }
}
