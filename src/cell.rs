use bevy::{math::IVec2, prelude::*};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    pub pos: IVec2,
    pub value: u8,
}

impl Cell {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            pos: IVec2::new(x, y),
            value: 0,
        }
    }
}
pub struct CellMaterials(pub Vec<Handle<ColorMaterial>>);

impl FromWorld for CellMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        CellMaterials(
            (0..=255)
                .map(|v| materials.add(Color::rgb_u8(v, v, v).into()))
                .collect(),
        )
    }
}
