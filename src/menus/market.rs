use crate::crop::attr::Crop;
use crate::objects::farm::Farm;

#[derive(Debug)]
pub struct FarmingMarket {
}

#[derive(Debug)]
pub struct Market {
    pub farming: FarmingMarket,
}

impl Market {
    pub fn new() -> Market {
        Market {
            farming: FarmingMarket::new(),
        }
    }
}

impl FarmingMarket {
    pub fn new() -> FarmingMarket {
        FarmingMarket {}
    }

    pub fn get_crop<T: Crop>(&self) -> Farm<T> {
        Farm::new(T::new())
    }
}