use rust_gpiozero::*;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let green = LED::new(18); // set the LED on pin 18 as "green" for the stop light
    let yellow = LED::new(13);
    let red = LED::new(22); // set a variable for the led on pin 18
    let mut count = 0;
    loop { // start an infinite loop
        green.on();
        sleep(Duration::from_secs(3)); // pause one second
        green.off();
        yellow.on();
        sleep(Duration::from_secs(2));
        yellow.off();
        red.on();
        sleep(Duration::from_secs(3));
        red.off();
        if count == 15 {
            green.off();
            yellow.off();
            red.off();
            break;
        }

        count = count + 1;
    }
}
