#![no_std]
#![no_main]
#![feature(generic_const_exprs)]

use crate::{
    display::{dvi::DviInterface, edid::read_edid, tmds::TMDS},
    graphics::camera::Camera,
    types::{
        matrix::Mat4,
        quat::Quaternion,
        vector::{VEC3_X, VEC3_Y},
    },
};
use defmt::info;
use esp_backtrace as _;
use esp_hal::main;
use esp_hal::{
    clock::CpuClock,
    i2c::master::{Config, I2c},
};
use esp_println as _;
use types::vector::VEC3_ZERO;

use {esp_backtrace as _, esp_println as _};

pub mod display;
pub mod graphics;
pub mod math;
pub mod model;
pub mod types;

pub type EdidBuffer = [u8; EDID_BUFFER_LEN];
pub const EDID_BUFFER_LEN: usize = 256;

#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    info!("{:?}", crate::model::CUBE_OBJ);

    let mut cam = Camera::<480, 360>::perspective(90.0, VEC3_X, -VEC3_X, VEC3_Y, 0.1, 50.0);
    info!("Created camera");

    let model = crate::model::model::from_obj(&crate::model::CUBE_OBJ);
    info!("Loaded model");

    let mut display = DviInterface {
        red_link: TMDS::new(peripherals.GPIO1, peripherals.GPIO2),
        green_link: TMDS::new(peripherals.GPIO3, peripherals.GPIO4),
        blue_link: TMDS::new(peripherals.GPIO12, peripherals.GPIO11),
        clock: TMDS::new(peripherals.GPIO10, peripherals.GPIO9),
        ddc: I2c::new(peripherals.I2C0, Config::default())
            .expect("config should be correct")
            .with_scl(peripherals.GPIO8)
            .with_sda(peripherals.GPIO7),
    };

    info!("Created monitor link");

    info!("Requesting EDID...");
    let mut edid_buffer: EdidBuffer = [0; 256];
    let _ = display.ddc.write_read(0x50, &[0xa1], &mut edid_buffer);

    read_edid(edid_buffer);

    info!("Beginning loop");

    loop {
        cam.render(
            &model,
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
            &mut display,
        );
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-beta.0/examples/src/bin
}
