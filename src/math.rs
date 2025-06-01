/// How many iterations of Heron's formula to run
const SQRT_ACCURACY: i32 = 6;

/// Calculates the square root of a float.
/// If the number is negative, returns 0.0
pub fn sqrt(num: f32) -> f32 {
    if num <= 0.0 {
        return 0.0;
    }
    let mut n: f32 = num / 2.0; // initial estimate
    for _ in 0..SQRT_ACCURACY {
        n = (n + num / n) / 2.0;
    }
    n
}
