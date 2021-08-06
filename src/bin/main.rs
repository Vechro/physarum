use bevy::prelude::*;
use physarum::agent::Agent;
use physarum::cell::CellMaterials;
use physarum::layers::trail_map::{setup_cells, TrailMap};

#[allow(dead_code)]
fn add_agents(mut commands: Commands) {
    for _ in 0..10 {
        commands.spawn().insert(Agent::default());
    }
}

#[allow(dead_code)]
fn list_agents(query: Query<&Agent>) {
    for agent in query.iter() {
        println!("agent.id: {}", agent.pos);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(1.0, 1.0, 1.0)))
        .init_resource::<TrailMap>()
        .init_resource::<CellMaterials>()
        // .add_startup_system(add_agents.system())
        .add_startup_system(setup_cells.system())
        .run();
}
