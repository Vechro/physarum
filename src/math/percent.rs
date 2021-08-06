#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Percent(pub f32);

impl Into<f32> for Percent {
    fn into(self) -> f32 {
        self.0 / 100f32
    }
}
