use bevy::{app::Events, ecs::system::Command, math::Vec2, prelude::*, tasks::{AsyncComputeTaskPool, ComputeTaskPool, TaskPoolBuilder}};
use rand::{thread_rng, Rng};

use crate::{
    board::Board,
    cell::{Cell, CellMaterials, CellUpdateEvent},
    math::{angle::Angle, vec2ext::Vec2Ext},
    triplet::{Prong, Triplet},
    AGENT_COUNT, CELL_SIZE, DEP_T, MAX, MIN, RA, SA, SO, SS,
};

#[derive(Debug, Clone, Copy)]
pub struct Agent {
    pub dir: Angle,
    pub pos: Vec2,
}

impl Default for Agent {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        Agent {
            dir: rng.gen::<Angle>(),
            pos: Vec2::new(
                rng.gen_range(MIN.x..MAX.x).into(),
                rng.gen_range(MIN.y..MAX.y).into(),
            ),
        }
    }
}

impl Agent {
    /// Returns FL, F and FR sensor angular directions
    fn sensor_directions(&self) -> (Angle, Angle, Angle) {
        (self.dir - SA, self.dir, self.dir + SA)
    }

    pub fn initialize(mut commands: Commands) {
        for _ in 0..*AGENT_COUNT {
            commands.spawn().insert(Agent::default());
        }
    }

    pub fn sense_and_move(
        pool: Res<ComputeTaskPool>,
        board: &'static Res<Board>,
        cell_mats: Res<CellMaterials>,
        mut commands: Commands,
        mut agent_query: Query<&mut Agent>,
        // mut event_writer: EventWriter<CellUpdateEvent>,
    ) {
        let mut events = Events::<CellUpdateEvent>::default();

        agent_query.par_for_each_mut(&pool, 64, |mut agent| {
            // lots of calculating
            let (left_heading, mid_heading, right_heading) = agent.sensor_directions();

            // TODO: Extract to function
            let left = agent.pos.produce_in_direction(left_heading, SO).as_i32();
            let mid = agent.pos.produce_in_direction(mid_heading, SO).as_i32();
            let right = agent.pos.produce_in_direction(right_heading, SO).as_i32();

            let triplet: Triplet<Option<&Entity>> = Triplet(
                board.map.get(&left),
                board.map.get(&mid),
                board.map.get(&right),
            );

            let prong = triplet.max();

            match prong {
                Prong::Left => agent.dir = agent.dir - RA,
                Prong::Middle => agent.dir = agent.dir,
                Prong::Right => agent.dir = agent.dir + RA,
            }

            let old_pos = agent.pos;
            let mut new_pos = agent.pos.produce_in_direction(agent.dir, SS);
            let mut new_dir = agent.dir;

            while !&new_pos.is_in_area(&MAX) {
                new_dir = thread_rng().gen::<Angle>();
                new_pos = new_pos.produce_in_direction(new_dir, SS);
            }

            agent.dir = new_dir;
            agent.pos = new_pos;

            // done with calculating! the agent has been moved into the new position,
            // now the old position must leave a trace of it on the grid, by updating the relevant cell

            if let Some(ent) = board.map.get(&old_pos.as_i32()) {
                // if let Ok(mut cell) = cell_query.get_component_mut::<Cell>(*ent) {
                //     cell.value += DEP_T;
                // }
                events.send(CellUpdateEvent {
                    cell_id: *ent,
                    increment_by: DEP_T,
                });
            }
        });
    }
}
