use bevy::{
    math::Vec2,
    prelude::*,
    tasks::{AsyncComputeTaskPool, ComputeTaskPool, TaskPoolBuilder},
};
use rand::{thread_rng, Rng};

use crate::{
    board::Board,
    cell::{Cell, CellMaterials},
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
        async_pool: Res<AsyncComputeTaskPool>,
        board: Res<Board>,
        cell_mats: Res<CellMaterials>,
        mut agent_query: Query<&'static mut Agent>,
        mut cell_query: Query<&'static mut Cell>,
    ) {
        let pool = TaskPoolBuilder::new()
            .thread_name("AgentThreadPool".to_string())
            .build();

        agent_query.for_each_mut(|mut agent| {
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

            let mut new_pos = agent.pos.produce_in_direction(agent.dir, SS);
            let mut new_dir = agent.dir;

            while !&new_pos.is_in_area(&MAX) {
                new_dir = thread_rng().gen::<Angle>();
                new_pos = new_pos.produce_in_direction(new_dir, SS);
            }

            agent.dir = new_dir;
            agent.pos = new_pos;

            if let Some(ent) = board.map.get(&new_pos.as_i32()) {
                if let Ok(mut cell) = cell_query.get_component_mut::<Cell>(*ent) {
                    cell.value += DEP_T
                }
            }
        });

        // pool.scope(|scope| {
        //     for i in 0..128 {
        //         scope.spawn(async move {});
        //     }
        // });
    }
}
