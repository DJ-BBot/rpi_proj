use rust_gpiozero::*;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let green = LED::new(18); // set the LED on pin 18 as "green" for the stop light
    let yellow = LED::new(13); // set the LED on pin 13 yellow for the stop light
    let red = LED::new(22); // set the LED on pin 22 red for the stop light
    let mut count = 0;

    let blues = [
        LED::new(23), // set the LED on pin 16 (GPIO 23)
        LED::new(25), // set the LED on pin 22 (GPIO 25)
        LED::new(12), // set the LED on pin 32 (GPIO 12)
        LED::new(16), // set the LED on pin 36 (GPIO 16)
        LED::new(5),
        LED::new(6),
        LED::new(19),
        LED::new(26)
    ];

    loop { // start an infinite loop
        light_cycle(&green, Duration::from_secs(3));
        light_cycle(&yellow, Duration::from_secs(2));
        light_cycle(&red, Duration::from_secs(3));
        if count == 2 {
            turn_off_lights(&green);
            turn_off_lights(&yellow);
            turn_off_lights(&red);
            break;
        }

        count = count + 1;
    }

    println!("Startup is complete!");

    for light in blues.iter() {
        light.on();
        sleep(Duration::from_secs(1));
    }
    for light in blues.iter() {
        light.off();
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
