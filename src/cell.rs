use bevy::{math::IVec2, prelude::*};
use lazy_static::lazy_static;

use crate::board::Board;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Cell<'a> {
    pub pos: IVec2,
    pub value: u8,
    pub neighbors: [Option<&'a Entity>; 8],
}

impl PartialOrd for Cell<'_> {
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

impl Cell<'_> {
    /// Neighbors must be added manually.
    pub fn new(pos: IVec2) -> Self {
        Self {
            pos,
            value: 0,
            neighbors: [None; 8],
        }
    }

    /// Call this after you've populated the board
    pub fn populate_neighbors(&self, board: &Board) {
        for (rel_pos, mut _neighbor) in RELATIVE_POSITIONS.iter().zip(self.neighbors) {
            _neighbor = board.map.get(&(self.pos + *rel_pos));
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
}

pub struct CellMaterials(pub Vec<Handle<ColorMaterial>>);

impl FromWorld for CellMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        CellMaterials(
            (0..=255)
                .map(|v: u8| materials.add(Color::rgba_u8(0, 0, 0, v).into()))
                .collect(),
        )
    }
}
