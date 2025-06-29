use defmt::info;
use esp_hal::{
    gpio::{Level, Output, OutputConfig, OutputPin},
    peripheral::Peripheral,
};

/// Manages a single TMDS (transition-minimised
/// differential signaling) pair.
pub struct TMDS<'a> {
    pin_nor: Output<'a>,
    pin_inv: Output<'a>,
    prev_disparity: i32,
}

impl<'a> TMDS<'a> {
    pub fn new(
        pin_nor: impl Peripheral<P = impl OutputPin> + 'a,
        pin_inv: impl Peripheral<P = impl OutputPin> + 'a,
    ) -> Self {
        TMDS {
            pin_nor: Output::new(pin_nor, Level::Low, OutputConfig::default()),
            pin_inv: Output::new(pin_inv, Level::Low, OutputConfig::default()),
            prev_disparity: 0,
        }
    }

    pub fn set_bit(&mut self, bit: bool) {
        self.pin_nor.set_level(bit.into());
        self.pin_inv.set_level((!bit).into());
    }

    pub fn toggle(&mut self) {
        self.pin_nor.toggle();
        self.pin_inv.toggle();
    }

    pub const fn encode_control_signal(c0: bool, c1: bool) -> u16 {
        match (c1, c0) {
            (false, false) => 0b0010101011,
            (false, true) => 0b1101010100,
            (true, false) => 0b0010101010,
            (true, true) => 0b1101010101,
        }
    }

    pub fn encode_tmds(&mut self, data: u8) -> u16 {
        let mut encoded: u8 = 0;
        let using_xor: bool;
        let bit_flip_occurred: bool;
        let mut num_ones = count_bits(data);

        encoded |= data & 1;

        // we choose either xor or xnor based on which one will give the fewest transitions
        if num_ones > 4 || (num_ones == 4 && (data & 0b10000000) >> 7 == 0) {
            // use xnor
            // out[1] = out[0] xnor in[1]
            for i in 1..8 {
                encoded |= !(get_bit(encoded, i - 1) ^ get_bit(data, i));
            }
            using_xor = false;
        } else {
            // use xor
            // out[1] = out[0] xor in[1]
            for i in 1..8 {
                encoded |= get_bit(encoded, i - 1) ^ get_bit(data, i);
            }
            using_xor = true;
        }

        num_ones = count_bits(encoded);

        if self.prev_disparity == 0 || num_ones == 4 {
            bit_flip_occurred = !using_xor;
            if !using_xor {
                encoded ^= 0xff;
                self.prev_disparity += <u8 as Into<i32>>::into(num_ones - (8 - num_ones));
            } else {
                self.prev_disparity += <u8 as Into<i32>>::into((8 - num_ones) - num_ones);
            }
        } else {
            if (self.prev_disparity > 0 && num_ones > 4)
                || (self.prev_disparity < 0 && num_ones < 4)
            {
                bit_flip_occurred = true;
                encoded ^= 0xff;
                self.prev_disparity += <u8 as Into<i32>>::into(
                    2 * <bool as Into<u8>>::into(using_xor) + num_ones - (8 - num_ones),
                );
            } else {
                bit_flip_occurred = false;
                self.prev_disparity += <u8 as Into<i32>>::into(
                    2 * <bool as Into<u8>>::into(using_xor) + (8 - num_ones) - num_ones,
                );
            }
        }

        // assemble the data
        let mut output: u16 = bit_flip_occurred.into();
        output <<= 1;
        output |= <bool as Into<u16>>::into(using_xor);
        output <<= 1;
        output |= <u8 as Into<u16>>::into(encoded);
        output
    }
}

/// Returns the number of 1 bits in a number.
fn count_bits(mut data: u8) -> u8 {
    let mut count: u8 = 0;
    for _ in 0..8 {
        if data & 1 == 1 {
            count += 1;
        }
        data >>= 1;
    }
    count
}

/// Gets the boolean value (0/1) of an indexed bit in an integer.
/// index is zero-based.
fn get_bit(data: u8, index: u8) -> u8 {
    data & (1 << index) >> index
}
