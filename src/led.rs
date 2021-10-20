use rust_gpiozero::*;
use std::thread::sleep;
use std::time::Duration;

pub struct RbLED {
    pub led: rust_gpiozero::LED,
    pub color: String,
    pub pin: u8,
}

impl RbLED {
    pub fn from(pin: u8, color: String) -> RbLED {
        RbLED {
            led: LED::new(pin),
            color: color,
            pin: pin,
        }
    }
    pub fn toggle(&self, secs: Duration) {
        println!("{} ON!", self.color);
        self.led.on();
        sleep(secs);
        self.led.off();
    }
    
    pub fn sw_off(&self) {
        if self.led.is_lit() {
            self.led.off();
        }
    }

    pub fn sw_on(&self) {
        if !self.led.is_lit() {
            println!("{} ON!", self.color);
            self.led.on();
        }
    }
}
