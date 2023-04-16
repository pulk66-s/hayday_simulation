use crate::objects::farm::Farm;
use crate::objects::types::BuildingType;
use crate::game::context::Context;
use crate::objects::build::Building;

const WIDTH: usize = 8;
const HEIGHT: usize = 8;

pub struct Board {
    pub cells: [[u8; WIDTH]; HEIGHT],
    pub farms: Vec<Farm>,
    pub buildings: Vec<BuildingType>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            cells: [[0; WIDTH]; HEIGHT],
            farms: Vec::new(),
            buildings: Vec::new(),
        }
    }
}

pub fn find_building(building_type: BuildingType, ctx: &mut Context) -> Option<&mut BuildingType> {
    for building in ctx.board.buildings.iter_mut() {
        if building.name() == building_type.name() {
            return Some(building);
        }
    }
    return None;
}
