use crate::objects::farm::Farm;
use crate::objects::coop::chicken::ChickenCoop;
use crate::animals::chicken::Chicken;

#[derive(Debug)]
pub struct FarmingMarket {
}

#[derive(Debug)]
pub struct AnimalMarket {
}

#[derive(Debug)]
pub struct Market {
    pub farming: FarmingMarket,
    pub animals: AnimalMarket,
}


impl Market {
    pub fn new() -> Market {
        Market {
            farming: FarmingMarket::new(),
            animals: AnimalMarket::new(),
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