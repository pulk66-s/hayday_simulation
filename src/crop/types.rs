use std::fmt;
use crate::crop::wheat::Wheat;

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