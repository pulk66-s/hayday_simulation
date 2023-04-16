use crate::crop::wheat::Wheat;

#[derive(Clone)]
pub enum CropType {
    Wheat(Wheat)
}