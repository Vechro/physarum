use std::iter::*;

use bevy::{math::Vec2, prelude::*, tasks::ComputeTaskPool};
use rand::{thread_rng, Rng};

use crate::{
    board::Board,
    cell::{CellUpdateEvent, CellUpdateEventCollection},
    math::{angle::Angle, vec2ext::Vec2Ext},
    triplet::{Prong, Triplet},
    AGENT_COUNT, DEP_T, MAX, MIN, RA, SA, SO, SS,
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
            commands
                .spawn()
                .insert(Agent::default())
                .insert(CellUpdateEventCollection::default());
        }
    }

    pub fn sense_and_move(
        pool: Res<ComputeTaskPool>,
        board: Res<Board>,
        mut query: Query<(&mut Agent, &mut CellUpdateEventCollection)>,
    ) {
        query.par_for_each_mut(&pool, 32, |(mut agent, mut events)| {
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

            agent.dir = match triplet.max() {
                Prong::Left => agent.dir - RA,
                Prong::Middle => agent.dir,
                Prong::Right => agent.dir + RA,
            };

            let mut new_pos = agent.pos.produce_in_direction(agent.dir, SS);

            while !&new_pos.is_in_area(&MAX) {
                let new_dir = thread_rng().gen::<Angle>();
                new_pos = new_pos.produce_in_direction(new_dir, SS);
            }

            // done with calculating!
            // now the old position must leave a trace of it on the grid,
            // by updating the relevant cell

            if let Some(ent) = board.map.get(&agent.pos.as_i32()) {
                events.0.send(CellUpdateEvent {
                    cell_id: *ent,
                    increment_by: DEP_T,
                });
            }

            agent.pos = new_pos;
        });
    }

    pub fn marshal_events(
        mut query: Query<&mut CellUpdateEventCollection>,
        mut event_writer: EventWriter<CellUpdateEvent>,
    ) {
        let batch =
            query
                .iter_mut()
                .fold(Vec::<CellUpdateEvent>::new(), |mut accum, mut events| {
                    accum.extend::<Vec<CellUpdateEvent>>(events.0.drain().collect());
                    // println!("{:?}", &accum);
                    accum
                });

        // println!("batch len: {:?}", batch.len());

        event_writer.send_batch(batch.into_iter());
    }
}
