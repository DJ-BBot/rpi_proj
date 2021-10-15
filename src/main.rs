use rust_gpiozero::*;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let green = LED::new(18); // set the LED on pin 18 as "green" for the stop light
    let yellow = LED::new(13); // set the LED on pin 13 yellow for the stop light
    let red = LED::new(22); // set the LED on pin 22 red for the stop light
    let mut count = 0;

    loop { // start an infinite loop
        light_cycle(&green, Duration::from_secs(3));
        light_cycle(&yellow, Duration::from_secs(2));
        light_cycle(&red, Duration::from_secs(3));
        if count == 15 {
            turn_off_lights(&green);
            turn_off_lights(&yellow);
            turn_off_lights(&red);
            break;
        }

        count = count + 1;
    }
}

fn light_cycle(lt: &rust_gpiozero::LED, secs: Duration) {
    lt.on();
    sleep(secs);
    lt.off();
}

fn turn_off_lights(lt: &rust_gpiozero::LED) {
    if lt.is_lit() {
        lt.off();
    }
}
