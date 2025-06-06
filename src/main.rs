#![no_std]
#![no_main]

use esp_alloc as _;
use esp_hal::clock::CpuClock;
use esp_hal::gpio::Level;
use esp_hal::gpio::Output;
use esp_hal::gpio::OutputConfig;
use esp_hal::main;
use esp_hal::time::{Duration, Instant};
use esp_println as _;
use log::info;

pub mod math;
pub mod model;
pub mod types;

#[main]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let _peripherals = esp_hal::init(config);

    let mut led = Output::new(_peripherals.GPIO7, Level::Low, OutputConfig::default());

    led.set_high();

    info!("Beginning loop");

    let model = crate::model::model::from_obj(crate::model::TEAPOT_OBJ);

    loop {
        led.toggle();
        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(5) {}
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-beta.0/examples/src/bin
}
