#![no_std]
#![no_main]

use defmt::info;
use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::main;
use esp_println as _;
use {esp_backtrace as _, esp_println as _};

pub mod display;
pub mod math;
pub mod model;
pub mod types;

#[main]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let _peripherals = esp_hal::init(config);

    info!("{:?}", crate::model::CUBE_OBJ);

    info!("Beginning loop");

    let _model = crate::model::model::from_obj(&crate::model::CUBE_OBJ);

    info!("Loaded model");

    loop {}

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-beta.0/examples/src/bin
}
