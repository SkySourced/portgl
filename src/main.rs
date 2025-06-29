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
use esp_hal::{
    clock::CpuClock,
    i2c::master::{Config, I2c},
    timer::{timg::TimerGroup, OneShotTimer},
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

#[esp_hal::main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let timg0 = TimerGroup::new(peripherals.TIMG0);

    let mut cam = Camera::<80, 60>::perspective(90.0, VEC3_X * 5.0, -VEC3_X, VEC3_Y, 0.1, 50.0);
    info!("Created camera");

    let model = crate::model::model::from_obj(&crate::model::CUBE_OBJ);
    info!("Loaded model");

    let mut display = DviInterface::new(
        peripherals.GPIO1.into(),
        peripherals.GPIO2.into(),
        peripherals.GPIO3.into(),
        peripherals.GPIO4.into(),
        peripherals.GPIO12.into(),
        peripherals.GPIO11.into(),
        peripherals.GPIO10.into(),
        peripherals.GPIO9.into(),
        peripherals.GPIO7.into(),
        peripherals.GPIO8.into(),
        peripherals.I2C0.into(),
        timg0.timer0.into(),
        timg0.timer1.into(),
    );

    // Set initial value for square wave inversion
    display.clock.set_bit(true);

    info!("Created monitor link");

    info!("Requesting EDID...");
    let mut edid_buffer: EdidBuffer = [0; 256];
    let _ = display.ddc.write_read(0x50, &[0xa1], &mut edid_buffer);

    read_edid(edid_buffer);

    info!("Beginning loop");

    cam.render(
        &model,
        Mat4::<f32>::idt(),
        &mut display,
    );

    loop {
        // cam.render_test(&mut display);
        
    }
}
