use core::usize;
use heapless::Vec;

/// Represents a render target.
pub struct FrameBuffer<T: Copy, const W: usize, const H: usize>
where
    [(); W * H]:,
{
    buffer: Vec<T, { W * H }>,
    // aspect: f32,
}

impl<T: Copy, const W: usize, const H: usize> FrameBuffer<T, W, H>
where
    [(); W * H]:,
{
    pub fn new() -> FrameBuffer<T, W, H>
    where
        [(); W * H]:,
    {
        FrameBuffer {
            buffer: Vec::new(),
            // aspect: W as f32 / H as f32,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> T {
        *self
            .buffer
            .get(y * W + x)
            .expect("coordinates specified should be within bounds of the framebuffer")
    }
}
