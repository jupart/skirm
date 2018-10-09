use std::time::Duration;

pub struct DeltaTime {
    pub delta: Duration,
}

impl DeltaTime {
    pub fn as_dt(&self) -> f32 {
        self.delta.as_secs() as f32 + self.delta.subsec_nanos() as f32 * 1e-9
    }
}
