use crate::objects::farm::Farm;
use crate::objects::coop::chicken::ChickenCoop;
use crate::animals::chicken::Chicken;
use crate::objects::provenderie::Provenderie;

#[derive(Debug, Clone)]
pub struct FarmingMarket {
}

#[derive(Debug, Clone)]
pub struct AnimalMarket {
}

#[derive(Debug, Clone)]
pub struct BuildingMarket {
}

#[derive(Debug, Clone)]
pub struct Market {
    pub farming: FarmingMarket,
    pub animals: AnimalMarket,
    pub buildings: BuildingMarket,
}


impl Market {
    pub fn new() -> Market {
        Market {
            farming: FarmingMarket::new(),
            animals: AnimalMarket::new(),
            buildings: BuildingMarket::new(),
        }
    }
}

impl FarmingMarket {
    pub fn new() -> FarmingMarket {
        FarmingMarket {}
    }

    pub fn get_farm(&self) -> Farm {
        Farm::new()
    }

    pub fn get_chicken_coop(&self) -> ChickenCoop {
        ChickenCoop::new()
    }
}

impl AnimalMarket {
    pub fn new() -> AnimalMarket {
        AnimalMarket {}
    }

    pub fn get_chicken(&self) -> Chicken {
        Chicken::new()
    }
}

impl BuildingMarket {
    pub fn new() -> BuildingMarket {
        BuildingMarket {}
    }

    pub fn get_provenderie(&self) -> Provenderie {
        Provenderie::new()
    }
}