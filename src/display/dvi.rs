use defmt::info;
use esp_hal::{
    gpio::{AnyPin, GpioPin}, i2c::master::{AnyI2c, I2c}, time::Duration, timer::{AnyTimer, OneShotTimer, PeriodicTimer}, Blocking, Config
};

use crate::{display::tmds::TMDS, types::vector::Vec3};

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
    pub line_clock: PeriodicTimer<'a, Blocking>,
}

const HSYNC_POLARITY: bool = false; // negative
const HSYNC_PULSE: u16 = TMDS::encode_control_signal(HSYNC_POLARITY, !VSYNC_POLARITY);

const VSYNC_POLARITY: bool = true; // positive
const VSYNC_PULSE: u16 = TMDS::encode_control_signal(!HSYNC_POLARITY, VSYNC_POLARITY);

const BSYNC_PULSE: u16 = TMDS::encode_control_signal(HSYNC_POLARITY, VSYNC_POLARITY);
const NSYNC_PULSE: u16 = TMDS::encode_control_signal(!HSYNC_POLARITY, !VSYNC_POLARITY);

impl<'a> DviInterface<'a> {
    pub fn new(
        red_nor: AnyPin,
        red_inv: AnyPin,
        green_nor: AnyPin,
        green_inv: AnyPin,
        blue_nor: AnyPin,
        blue_inv: AnyPin,
        clock_nor: AnyPin,
        clock_inv: AnyPin,
        sda: AnyPin,
        scl: AnyPin,
        ddc: AnyI2c,
        timg0: AnyTimer,
        timg1: AnyTimer,
    ) -> DviInterface<'a> {
        let mut line_clock: PeriodicTimer<'_, Blocking> = PeriodicTimer::new(timg0);
        line_clock.start(Duration::from_micros(45));
        DviInterface {
            red_link: TMDS::new(red_nor, red_inv),
            green_link: TMDS::new(green_nor, green_inv),
            blue_link: TMDS::new(blue_nor, blue_inv),
            clock: TMDS::new(clock_nor, clock_inv),
            ddc: I2c::new(ddc, esp_hal::i2c::master::Config::default())
                .expect("config should be correct")
                .with_scl(scl)
                .with_sda(sda),
            line_clock,
        }
    }

    pub fn render_pixel(&mut self, pixel: Vec3<u8>) {
        let red_tmds = self.red_link.encode_tmds(pixel.x);
        let green_tmds = self.green_link.encode_tmds(pixel.y);
        let blue_tmds = self.blue_link.encode_tmds(pixel.z);
        self.clock.toggle();
        for i in 0..10 {
            self.red_link.set_bit((red_tmds >> i) & 1 == 1);
            self.green_link.set_bit((green_tmds >> i) & 1 == 1);
            self.blue_link.set_bit((blue_tmds >> i) & 1 == 1);
            if i == 5 {
                self.clock.toggle();
            }
        }
    }

    pub fn end_row(&mut self) {
        // FRONT PORCH
        for _ in 0..16 {
            self.clock.toggle();
        }

        // SYNC PERIOD
        for _ in 0..40 {
            self.clock.toggle();
            for i in 0..10 {
                self.red_link.set_bit((HSYNC_PULSE >> i) & 1 == 1);
                self.green_link.set_bit((HSYNC_PULSE >> i) & 1 == 1);
                self.blue_link.set_bit((HSYNC_PULSE >> i) & 1 == 1);
                if i == 5 {
                    self.clock.toggle();
                }
            }
        }

        // BACK PORCH
        for _ in 0..56 {
            self.clock.toggle();
        }
    }

    pub fn end_frame(&mut self) {
        // FRONT PORCH
        for _ in 0..3 {
            self.end_line(false);
            self.line_clock.wait();
        }

        // SYNC PERIOD
        for _ in 0..4 {
            self.end_line(true);
            self.line_clock.wait();
        }

        // BACK PORCH
        for _ in 0..9 {
            self.end_line(false);
            self.line_clock.wait();
        }
    }

    /// Fake line for vertical blanking
    fn end_line(&mut self, vsync: bool) {
        // Data period
        for _ in 0..360 {
            self.clock.toggle();
                for i in 0..10 {
                    self.red_link.set_bit((HSYNC_PULSE >> i) & 1 == 1);
                    self.green_link.set_bit((HSYNC_PULSE >> i) & 1 == 1);
                    self.blue_link.set_bit((HSYNC_PULSE >> i) & 1 == 1);
                    if i == 5 {
                        self.clock.toggle();
                    }
                }
        }

        // front porch
        if vsync {
            for _ in 0..16 {
                self.clock.toggle();
                for i in 0..10 {
                    self.red_link.set_bit((VSYNC_PULSE >> i) & 1 == 1);
                    self.green_link.set_bit((VSYNC_PULSE >> i) & 1 == 1);
                    self.blue_link.set_bit((VSYNC_PULSE >> i) & 1 == 1);
                    if i == 5 {
                        self.clock.toggle();
                    }
                }
            }
        } else {
            for _ in 0..16 {
                self.clock.toggle();
                for i in 0..10 {
                    self.red_link.set_bit((NSYNC_PULSE >> i) & 1 == 1);
                    self.green_link.set_bit((NSYNC_PULSE >> i) & 1 == 1);
                    self.blue_link.set_bit((NSYNC_PULSE >> i) & 1 == 1);
                    if i == 5 {
                        self.clock.toggle();
                    }
                }
            }
        }

        // sync pulse
        if vsync {
            for _ in 0..40 {
                self.clock.toggle();
                for i in 0..10 {
                    self.red_link.set_bit((BSYNC_PULSE >> i) & 1 == 1);
                    self.green_link.set_bit((BSYNC_PULSE >> i) & 1 == 1);
                    self.blue_link.set_bit((BSYNC_PULSE >> i) & 1 == 1);
                    if i == 5 {
                        self.clock.toggle();
                    }
                }
            }
        } else {
            for _ in 0..40 {
                self.clock.toggle();
                for i in 0..10 {
                    self.red_link.set_bit((HSYNC_PULSE >> i) & 1 == 1);
                    self.green_link.set_bit((HSYNC_PULSE >> i) & 1 == 1);
                    self.blue_link.set_bit((HSYNC_PULSE >> i) & 1 == 1);
                    if i == 5 {
                        self.clock.toggle();
                    }
                }
            }
        }

        // back porch
        if vsync {
            for _ in 0..56 {
                self.clock.toggle();
                for i in 0..10 {
                    self.red_link.set_bit((VSYNC_PULSE >> i) & 1 == 1);
                    self.green_link.set_bit((VSYNC_PULSE >> i) & 1 == 1);
                    self.blue_link.set_bit((VSYNC_PULSE >> i) & 1 == 1);
                    if i == 5 {
                        self.clock.toggle();
                    }
                }
            }
        } else {
            for _ in 0..56 {
                self.clock.toggle();
                for i in 0..10 {
                    self.red_link.set_bit((NSYNC_PULSE >> i) & 1 == 1);
                    self.green_link.set_bit((NSYNC_PULSE >> i) & 1 == 1);
                    self.blue_link.set_bit((NSYNC_PULSE >> i) & 1 == 1);
                    if i == 5 {
                        self.clock.toggle();
                    }
                }
            }
        }
    }
}
