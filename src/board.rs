use bevy::{math::IVec2, prelude::*, utils::AHashExt, utils::HashMap};

use crate::{
    cell::{Cell, CellMaterials},
    CELL_SIZE, DIMENSIONS, MAX, MIN,
};

#[derive(Debug, Clone)]
pub struct Board {
    pub width: i32,
    pub height: i32,
    pub map: HashMap<IVec2, Entity>,
}

impl Default for Board {
    fn default() -> Self {
        Board {
            width: DIMENSIONS.x as i32,
            height: DIMENSIONS.y as i32,
            map: HashMap::with_capacity(DIMENSIONS.x as usize * DIMENSIONS.y as usize),
        }
    }
}

impl Board {
    pub fn length(&self) -> i32 {
        self.width * self.height
    }

    pub fn index_to_pos(&self, index: i32) -> IVec2 {
        let x = index % self.width;
        let y = index / self.width;

        IVec2::new(x, y)
    }

    pub fn pos_to_index(&self, pos: IVec2) -> i32 {
        pos.y * self.width + pos.x
    }

    pub fn initialize(
        mut board: ResMut<Board>,
        mut commands: Commands,
        cell_mats: Res<CellMaterials>,
    ) {
        for x in (MIN.x as i32..MAX.x as i32).step_by(CELL_SIZE.x as usize) {
            for y in (MIN.y as i32..MAX.y as i32).step_by(CELL_SIZE.y as usize) {
                let pos = IVec2::new(x, y);
                let cell = Cell::new(pos);

                let mut entity_c = commands.spawn();
                board.map.insert(pos, entity_c.id());

                entity_c
                    .insert_bundle(SpriteBundle {
                        material: cell_mats.0[0].clone(),
                        transform: Transform::from_translation(pos.as_f32().extend(0.0)),
                        sprite: Sprite::new(*CELL_SIZE),
                        ..Default::default()
                    })
                    .insert(cell);
            }
        }
    }

    pub fn register_neighbors(board: Res<Board>, mut cell_q: Query<&mut Cell>) {
        cell_q.for_each_mut(|cell| {
            cell.populate_neighbors(&board);
        })
    }
}
