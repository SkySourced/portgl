use heapless::Vec;

use crate::types::vector::{Vec2, Vec3};

pub struct Texture<const W: usize, const H: usize>
where
    [(); W * H]:,
{
    buffer: Vec<Vec3<u8>,{ W * H }>,
}

impl<const W: usize, const H: usize> Texture<W, H>
where
    [(); W * H]:,
{
    pub fn sample_pixels(&self, x: usize, y: usize) -> Option<Vec3<u8>> {
        self.buffer.get(y * H + x).copied()
    }

    pub fn sample_f(&self, x: f32, y: f32) -> Option<Vec3<u8>> {
        self.buffer
            .get((y * H as f32) as usize * H + (x * W as f32) as usize)
            .copied()
    }

    pub fn sample(&self, tex_coords: Vec2<f32>) -> Option<Vec3<u8>> {
        Self::sample_f(self, tex_coords.x, tex_coords.y)
    }

    pub fn gen_checkerboard() -> Texture<W, H> {
        let mut tex: Texture<W, H> = Texture { buffer: Vec::new() };
        for y in 0..H {
            for x in 0..W {
                if x / 8 % 2 == 1 && y / 8 % 2 == 1 {
                    let _ = tex.buffer.push(Vec3 { x: 255, y: 255, z: 255 });
                } else {
                    let _ = tex.buffer.push(Vec3 { x: 0, y: 0, z: 0 });
                }
            }
        }
        tex
    }
}
