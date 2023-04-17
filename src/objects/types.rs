use crate::crop::wheat::Wheat;
use crate::objects::{
    farm::Farm,
    silo::Silo,
    barn::Barn,
    provenderie::Provenderie,
    coop::chicken::ChickenCoop,
    animal_food::chicken::ChickenFood,
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
    ChickenFood(ChickenFood)
}

#[derive(Clone, Debug)]
pub enum BuildingType {
    Farm(Farm),
    Silo(Silo),
    Barn(Barn),
    ChickenCoop(ChickenCoop),
    Provenderie(Provenderie),
}

impl Building for BuildingType {
    fn build(&self, ctx: &mut Context) -> bool {
        match self {
            BuildingType::Farm(farm) => farm.build(ctx),
            BuildingType::Silo(silo) => silo.build(ctx),
            BuildingType::Barn(barn) => barn.build(ctx),
            BuildingType::ChickenCoop(coop) => coop.build(ctx),
            BuildingType::Provenderie(provenderie) => provenderie.build(ctx),
        }
    }

    fn name(&self) -> String {
        match self {
            BuildingType::Farm(farm) => farm.name(),
            BuildingType::Silo(silo) => silo.name(),
            BuildingType::Barn(barn) => barn.name(),
            BuildingType::ChickenCoop(coop) => coop.name(),
            BuildingType::Provenderie(provenderie) => provenderie.name(),
        }
    }
}

impl fmt::Display for BuildingType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BuildingType::Farm(farm) => write!(f, "{}", farm),
            BuildingType::Silo(silo) => write!(f, "{}", silo),
            BuildingType::Barn(barn) => write!(f, "{}", barn),
            BuildingType::ChickenCoop(coop) => write!(f, "{}", coop),
            BuildingType::Provenderie(provenderie) => write!(f, "{}", provenderie),
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
            BarnContent::ChickenFood(chicken_food) => write!(f, "{}", chicken_food),
            _ => write!(f, "BarnContent"),
        }
    }
}
