pub struct MathHelper;

impl MathHelper {
    pub fn round(x : f32, n : i32) -> f32{
        (x / (n as f32 * 10.0)).floor() * (n as f32 * 10.0)
    }
}