#![no_std]
#![no_main]

use defmt::info;
use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::main;
use esp_println as _;
use types::vector::VEC3_ZERO;
use {esp_backtrace as _, esp_println as _};

pub mod display;
pub mod math;
pub mod model;
pub mod types;

const VIEWPORT_WIDTH: u16 = 320;
const VIEWPORT_HEIGHT: u16 = 240;

#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let _peripherals = esp_hal::init(config);

    info!("{:?}", crate::model::CUBE_OBJ);

    info!("Beginning loop");

    let model = crate::model::model::from_obj(&crate::model::CUBE_OBJ);

    info!("Loaded model");

    model.render(VEC3_ZERO);

    loop {}

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-beta.0/examples/src/bin
}
