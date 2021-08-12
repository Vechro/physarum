use bevy::{core::FixedTimesteps, prelude::*};

pub const LABEL_TIMESTEP: &str = "Fixed Timestep";

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub struct FixedUpdateStage;

pub fn frame_update(mut last_time: Local<f64>, time: Res<Time>) {
    info!("update: {}", time.seconds_since_startup() - *last_time);
    *last_time = time.seconds_since_startup();
}

pub fn fixed_update(mut last_time: Local<f64>, time: Res<Time>, fixed_timesteps: Res<FixedTimesteps>) {
    info!(
        "fixed_update: {}",
        time.seconds_since_startup() - *last_time,
    );

    let fixed_timestep = fixed_timesteps.get(LABEL_TIMESTEP).unwrap();
    info!(
        "  overstep_percentage: {}",
        fixed_timestep.overstep_percentage()
    );

    *last_time = time.seconds_since_startup();
}
