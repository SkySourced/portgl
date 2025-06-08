#![no_std]
#![no_main]

use esp_alloc as _;
use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::gpio::Level;
use esp_hal::gpio::Output;
use esp_hal::gpio::OutputConfig;
use esp_hal::main;
use esp_println as _;
use log::info;

pub mod display;
pub mod math;
pub mod model;
pub mod types;

#[main]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let _peripherals = esp_hal::init(config);

    let mut led: Output<'_> = Output::new(_peripherals.GPIO7, Level::Low, OutputConfig::default());

    led.set_high();

    info!("{:?}", crate::model::CUBE_OBJ);

    info!("Beginning loop");

    let _model = crate::model::model::from_obj(&crate::model::CUBE_OBJ);

    info!("Loaded model");

    loop {}

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-beta.0/examples/src/bin
}
