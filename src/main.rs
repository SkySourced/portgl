#![no_std]
#![no_main]

use crate::{
    graphics::camera::perspective,
    types::{
        quat::Quaternion,
        vector::{VEC3_X, VEC3_Y},
        matrix::{Mat4}
    },
};
use defmt::info;
use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::main;
use esp_println as _;
use types::vector::VEC3_ZERO;

use {esp_backtrace as _, esp_println as _};

pub mod display;
pub mod graphics;
pub mod math;
pub mod model;
pub mod types;

#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let _peripherals = esp_hal::init(config);

    info!("{:?}", crate::model::CUBE_OBJ);

    info!("Beginning loop");

    let cam = perspective(90.0, VEC3_X, -VEC3_X, VEC3_Y, 0.1, 50.0);
    let model = crate::model::model::from_obj(&crate::model::CUBE_OBJ);

    info!("Loaded model");

    cam.render(
        model,
        Mat4::<f32>::transform(
            VEC3_ZERO,
            Quaternion {
                a: 1.0,
                i: 0.0,
                j: 0.0,
                k: 0.0,
            },
            1.0,
        ),
    );

    loop {}

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-beta.0/examples/src/bin
}
