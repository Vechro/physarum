use arrayvec::ArrayVec;
use bevy::{app::Events, math::IVec2, prelude::*};
use lazy_static::lazy_static;

use crate::board::Board;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Cell {
    pub pos: IVec2,
    pub value: u8,
    pub neighbors: ArrayVec<Entity, 8>,
}

impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.value.cmp(&other.value))
    }
}

lazy_static! {
    #[rustfmt::skip]
    static ref RELATIVE_POSITIONS: [IVec2; 8] = [
        IVec2::new(-1, -1), IVec2::new(0, -1), IVec2::new(1, -1),
        IVec2::new(-1,  0),                    IVec2::new(1,  0),
        IVec2::new(-1,  1), IVec2::new(0,  1), IVec2::new(1,  1),
    ];
}

impl Cell {
    /// Neighbors must be added manually.
    pub fn new(pos: IVec2) -> Self {
        Self {
            pos,
            value: 0,
            neighbors: ArrayVec::new(),
        }
    }

    /// Call this after you've populated the board
    pub fn populate_neighbors(&self, board: &Board) {
        for (rel_pos, mut _neighbor) in RELATIVE_POSITIONS.iter().zip(&self.neighbors) {
            if let Some(entity) = board.map.get(&(self.pos + *rel_pos)) {
                _neighbor = entity
            }
        }
    }

    // TODO
    // pub fn decay(
    //     mut commands: Commands,
    //     cell_mats: Res<CellMaterials>,
    //     mut cell_query: Query<&'static Cell>,
    // ) {
    //     cell_query.for_each_mut(|mut cell| {})
    // }

    pub fn update_listener(
        mut events: EventReader<CellUpdateEvent>,
        mut q: Query<(&mut Cell, &Handle<ColorMaterial>)>,
        // cell_mats: ResMut<CellMaterials>,
        mut mats: ResMut<Assets<ColorMaterial>>,
    ) {
        for cell_update_event in events.iter() {
            if let Ok(mut cell) = q.get_component_mut::<Cell>(cell_update_event.cell_id) {
                let val = cell.value.saturating_add(cell_update_event.increment_by);
                cell.value = val;

                if let Ok(mat_handle) =
                    q.get_component::<Handle<ColorMaterial>>(cell_update_event.cell_id)
                {
                    if let Some(mut m) = mats.get_mut(mat_handle) {
                        m.color = Color::rgba_u8(0, 0, 0, val);
                    }
                }
            }
        }
    }
}

// pub struct CellMaterials(pub Vec<Handle<ColorMaterial>>);

// impl FromWorld for CellMaterials {
//     fn from_world(world: &mut World) -> Self {
//         let mut materials =
// world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
//         CellMaterials(
//             (0..=255)
//                 .map(|v: u8| materials.add(Color::rgba_u8(0, 0, 0,
// v).into()))                 .collect(),
//         )
//     }
// }

#[derive(Debug, Clone, Copy)]
pub struct CellUpdateEvent {
    pub cell_id: Entity,
    pub increment_by: u8,
}

#[derive(Debug, Default)]
pub struct CellUpdateEventCollection(pub Events<CellUpdateEvent>);
