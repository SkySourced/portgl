use crate::{display::tmds::TMDS, graphics::viewport::FrameBuffer};

/// Represents a single set of three TMDS pairs.
/// DVI Single Link has only one, but Dual Link
/// has two.
/// The potentially shared clock pair is separate.
pub struct DviInterface<'a> {
    pub red_link: TMDS<'a>,
    pub green_link: TMDS<'a>,
    pub blue_link: TMDS<'a>,
    pub clock: TMDS<'a>,
}

impl<'a> DviInterface<'a> {
    pub fn render<const W: usize, const H: usize>(&self, fbo: &FrameBuffer<W, H>)
    where
        [(); W * H]:,
    {
    }
}
