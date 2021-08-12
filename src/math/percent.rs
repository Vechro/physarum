#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Percent(pub f32);

impl Into<f32> for Percent {
    fn into(self) -> f32 {
        self.0 / 100f32
    }
}

impl Into<u32> for Percent {
    fn into(self) -> u32 {
        let prim: f32 = self.into();
        prim as u32
    }
}
