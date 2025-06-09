use crate::display::tmds::TMDS;

/// Represents a single set of three TMDS pairs.
/// DVI Single Link has only one, but Dual Link
/// has two.
/// The potentially shared clock pair is separate.
pub struct DviInterface<'a> {
    red_link: TMDS<'a>,
    green_link: TMDS<'a>,
    blue_link: TMDS<'a>,
    clock: &'a TMDS<'a>,
}
