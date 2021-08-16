use bevy::{
    core::FixedTimestep,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use physarum::{
    agent::Agent,
    board::Board,
    cell::{Cell, CellMaterials, CellUpdateEvent},
    DIMENSIONS,
};

pub const LABEL_TIMESTEP: &str = "Fixed Timestep";

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, StageLabel)]
pub struct FixedUpdateStage;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: DIMENSIONS.x,
            height: DIMENSIONS.y,
            // scale_factor_override: Some(1.0),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        // Adds frame time diagnostics
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // Adds a system that prints diagnostics to the console
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(bevy::diagnostic::EntityCountDiagnosticsPlugin::default())
        .add_plugin(bevy::asset::diagnostic::AssetCountDiagnosticsPlugin::<ColorMaterial>::default())
        .insert_resource(ClearColor(Color::rgb(1.0, 1.0, 1.0)))
        .init_resource::<CellMaterials>()
        .init_resource::<Board>()
        .add_event::<CellUpdateEvent>()
        .add_startup_system(setup)
        .add_startup_system(Board::initialize)
        .add_startup_system(Board::register_neighbors)
        .add_startup_system(Agent::initialize)
        .add_stage_after(
            CoreStage::Update,
            FixedUpdateStage,
            SystemStage::parallel()
                .with_run_criteria(
                    FixedTimestep::step(0.667)
                )
                .with_system(Agent::sense_and_move),
        )
        // .add_system(Agent::sense_and_move)
        .add_system(Agent::marshal_events)
        .add_system(Cell::update_listener)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
