use bevy::{core::FixedTimestep, prelude::*};
use physarum::{
    agent::Agent,
    board::Board,
    cell::CellMaterials,
    timestep::{FixedUpdateStage, LABEL_TIMESTEP},
    DIMENSIONS,
};

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: DIMENSIONS.x,
            height: DIMENSIONS.y,
            // scale_factor_override: Some(1.0),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(1.0, 1.0, 1.0)))
        .init_resource::<CellMaterials>()
        .init_resource::<Board>()
        .add_startup_system(setup)
        .add_startup_system(Board::initialize)
        .add_startup_system(Board::register_neighbors)
        .add_startup_system(Agent::initialize)
        .add_stage_after(
            CoreStage::Update,
            FixedUpdateStage,
            SystemStage::parallel()
                .with_run_criteria(
                    FixedTimestep::step(2.5)
                        // labels are optional. they provide a way to access the current
                        // FixedTimestep state from within a system
                        .with_label(LABEL_TIMESTEP),
                )
                // .with_system(fixed_update)
                .with_system(Agent::sense_and_move),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
