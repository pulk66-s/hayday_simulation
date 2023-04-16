use crate::crop::wheat::Wheat;
use crate::objects::{
    farm::Farm,
    silo::Silo,
    barn::Barn,
};
use crate::game::context::Context;
use crate::objects::build::Building;
use std::fmt;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum SiloContent {
    Wheat(Wheat),
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum BarnContent {
}

#[derive(Clone)]
pub enum BuildingType {
    Farm(Farm),
    Silo(Silo),
    Barn(Barn)
}

impl Building for BuildingType {
    fn build(&self, ctx: &mut Context) -> bool {
        match self {
            BuildingType::Farm(farm) => farm.build(ctx),
            BuildingType::Silo(silo) => silo.build(ctx),
            BuildingType::Barn(barn) => barn.build(ctx),
        }
    }

    fn name(&self) -> String {
        match self {
            BuildingType::Farm(farm) => farm.name(),
            BuildingType::Silo(silo) => silo.name(),
            BuildingType::Barn(barn) => barn.name(),
        }
    }
}

impl fmt::Display for BuildingType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BuildingType::Farm(farm) => write!(f, "{}", farm),
            BuildingType::Silo(silo) => write!(f, "{}", silo),
            BuildingType::Barn(barn) => write!(f, "{}", barn),
        }
    }
}

impl fmt::Display for SiloContent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SiloContent::Wheat(wheat) => write!(f, "{}", wheat),
        }
    }
}

impl fmt::Display for BarnContent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            _ => write!(f, "BarnContent"),
        }
    }
}
