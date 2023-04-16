use std::fmt;
use crate::crop::wheat::Wheat;

pub trait Crop {
    fn duration(&self) -> u32;
    fn name(&self) -> String;
}

#[derive(Clone)]
pub enum CropType {
    Wheat(Wheat)
}

impl fmt::Display for CropType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CropType::Wheat(wheat) => write!(f, "Wheat: {}", wheat),
        }
    }
}

impl Crop for CropType {
    fn duration(&self) -> u32 {
        match self {
            CropType::Wheat(wheat) => wheat.duration,
        }
    }

    fn name(&self) -> String {
        match self {
            CropType::Wheat(wheat) => wheat.name.clone(),
        }
    }
}