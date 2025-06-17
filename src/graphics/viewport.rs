use crate::types::vector::Vec3;
use core::usize;
use heapless::Vec;

/// Represents a render target.
pub struct FrameBuffer<const W: usize, const H: usize>
where
    [(); W * H]:,
{
    buffer: Vec<Vec3<f32>, { W * H }>,
    aspect: f32,
}

impl<const W: usize, const H: usize> FrameBuffer<W, H>
where
    [(); W * H]:,
{
    pub fn new() -> FrameBuffer<W, H>
    where
        [(); W * H]:,
    {
        FrameBuffer {
            buffer: Vec::new(),
            aspect: W as f32 / H as f32,
        }
    }
}
