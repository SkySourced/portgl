use heapless::Vec;

pub struct Texture<T: Copy, const W: usize, const H: usize>
where
    [(); W * H]:,
{
    buffer: Vec<T, { W * H }>,
}

impl<T: Copy, const W: usize, const H: usize> Texture<T, W, H>
where
    [(); W * H]:,
{
    pub fn sample_pixels(&self, x: usize, y: usize) -> Option<T> {
        self.buffer.get(y * H + x).copied()
    }

    pub fn sample(&self, x: f32, y: f32) -> Option<T> {
        self.buffer
            .get((y * H as f32) as usize * H + (x * W as f32) as usize)
            .copied()
    }
}
