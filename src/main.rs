use std::thread::sleep;
use std::time::Duration;

use rppal::gpio::Gpio;
use rppal::gpio::Level;
use rppal::gpio::Mode;
use rppal::system::DeviceInfo;
// The `Gpio` module uses BCM pin numbering. BCM GPIO 18 is tied to physical pin 12.
const GPIO_LED: u8 = 18;

fn main() {
    let device_info = DeviceInfo::new().unwrap();
    println!(
        "Model: {} (SoC: {})",
        device_info.model(),
        device_info.soc()
    );

    let mut gpio = Gpio::new().unwrap();
    gpio.set_mode(GPIO_LED, Mode::Output);

    // Blink an LED attached to the pin.
    gpio.write(GPIO_LED, Level::High);
    sleep(Duration::from_millis(500));
    gpio.write(GPIO_LED, Level::Low);
}
