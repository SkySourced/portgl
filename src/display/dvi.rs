use esp_hal::{i2c::master::I2c, Blocking};

use crate::{display::tmds::TMDS, graphics::viewport::FrameBuffer, types::vector::Vec3};

/// Represents a single set of three TMDS pairs.
/// DVI Single Link has only one, but Dual Link
/// has two.
/// The potentially shared clock pair is separate.
pub struct DviInterface<'a> {
    pub red_link: TMDS<'a>,
    pub green_link: TMDS<'a>,
    pub blue_link: TMDS<'a>,
    pub clock: TMDS<'a>,
    pub ddc: I2c<'a, Blocking>,
}

impl<'a> DviInterface<'a> {
    pub fn render<const W: usize, const H: usize>(&mut self, fbo: &FrameBuffer<Vec3<u8>, W, H>)
    where
        [(); W * H]:,
    {
        for y in 0..H {
            for x in 0..W {
                let pixel_color: Vec3<u8> = fbo.get(x, y);
                self.red_link.send_byte(pixel_color.x);
                self.green_link.send_byte(pixel_color.y);
                self.blue_link.send_byte(pixel_color.z);
            }
        }
    }

    pub fn render_pixel(&mut self, pixel: Vec3<u8>) {
        self.red_link.send_byte(pixel.x);
        self.green_link.send_byte(pixel.y);
        self.blue_link.send_byte(pixel.z);
    }

    pub fn end_row(&self) {}

    pub fn end_frame(&self) {}
}
