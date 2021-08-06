use bevy::prelude::*;

use crate::{
    cell::{Cell, CellMaterials},
    CELL_SIZE, HEIGHT, MAX_X, MAX_Y, MIN_X, MIN_Y, WIDTH,
};

#[derive(Debug, Clone, PartialEq)]
pub struct TrailMap {
    pub width: u32,
    pub height: u32,
    pub map: Vec<Cell>,
}

impl Default for TrailMap {
    fn default() -> Self {
        TrailMap::new(WIDTH.into(), HEIGHT.into())
    }
}

impl TrailMap {
    pub fn new(width: u32, height: u32) -> Self {
        let min_x = width as i32 / -2;
        let max_x = width as i32 / 2;
        let min_y = height as i32 / -2;
        let max_y = height as i32 / 2;

        let mut map: Vec<Cell> = Default::default();

        for x in min_x..max_x {
            for y in min_y..max_y {
                map.push(Cell::new(x, y))
            }
        }
        Self { width, height, map }
    }

    fn index_to_coords(&self, index: u32) -> (i32, i32) {
        let y = index / self.width;
        let x = index - y * self.width;
        (x as i32 / -2, y as i32 / -2)
    }

    pub fn find_by_coords(&self, x: i32, y: i32) -> Option<&Cell> {
        let half_width = self.width as i32 / 2;
        let half_height = self.height as i32 / 2;

        let ux = (x + half_width) as u32;
        let uy = (y + half_height) as u32;
        let index = (uy * self.height + ux) as usize;

        // FIXME: Incorrect result
        self.map.get(index)
    }
}

pub fn setup_cells(mut commands: Commands, cell_mats: Res<CellMaterials>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    for x in MIN_X..MAX_X {
        for y in MIN_Y..MAX_Y {
            commands
                .spawn_bundle(SpriteBundle {
                    material: cell_mats.0[0usize].clone(),
                    transform: Transform::from_xyz(
                        (x * (CELL_SIZE + 2) as i16) as f32,
                        (y * (CELL_SIZE + 2) as i16) as f32,
                        0.0,
                    ),
                    sprite: Sprite::new(Vec2::splat(CELL_SIZE as f32)),
                    ..Default::default()
                })
                .insert(Cell::new(x.into(), y.into()));
        }
    }
}
