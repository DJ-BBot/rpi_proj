use rust_gpiozero::*;
use std::thread::sleep;
use std::time::Duration;

// The RbLED struct holds the useful fields for the LEDs that I want to hold
pub struct RbLED {
    pub led: rust_gpiozero::LED, //the actual LED object
    pub color: String, //the color of the physical LED
    pub pin: u8, //the GPIO pin number
}

impl RbLED {
    // Construct a new LED given parameters
    pub fn from(pin: u8, color: String) -> RbLED {
        RbLED {
            led: LED::new(pin),
            color: color,
            pin: pin,
        }
    }

    // Print the LED parameters
    fn print_led(&self) {
        let status = match self.led.is_lit() {
            true => String::from("Active"),
            false => String::from("Inactive"),
        };
        println!("LED:\n\t  Status:\t{}\n\tColor:\t  {}\n\tGPIO Pin: {}\n", 
            status, self.color, self.pin);
    }

    // Turn the LED on and off for a set time
    pub fn toggle(&self, secs: Duration) {
        self.sw_on();
        sleep(secs);
        self.sw_off();
    }
    
    // Switch off the LED
    pub fn sw_off(&self) {
        if self.led.is_lit() {
            self.led.off();
        }
        self.print_led();
    }

    // Switch on the LED
    pub fn sw_on(&self) {
        if !self.led.is_lit() {
            self.led.on();
        }
        self.print_led();
    }
}
