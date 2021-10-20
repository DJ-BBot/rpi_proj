use std::thread::sleep;
use std::time::Duration;

mod led;

fn main() {
    let g_led = led::RbLED::from(17, String::from("Green"));
    let y_led = led::RbLED::from(13, String::from("Yellow"));
    let r_led = led::RbLED::from(22, String::from("Red"));

    let blues = [
        led::RbLED::from(23, String::from("Blue")),
        led::RbLED::from(25, String::from("Blue")),
        led::RbLED::from(12, String::from("Blue")),
        led::RbLED::from(16, String::from("Blue")),
        led::RbLED::from(5,  String::from("Blue")), 
        led::RbLED::from(6,  String::from("Blue")), 
        led::RbLED::from(24, String::from("Blue")),
        led::RbLED::from(26, String::from("Blue")),
    ];

    for _loop_cntr in 1..3 {
        g_led.toggle(Duration::from_secs(1));
        y_led.toggle(Duration::from_secs(1));
        r_led.toggle(Duration::from_secs(1));
    }

    println!("Startup is complete!");
    // turn on the blue lights
    for light in blues.iter() {
        light.sw_on();
        sleep(Duration::from_secs(1));
    }

    // turn off the blue lights
    for light in blues.iter() {
        light.sw_off();
    }
}

