use crate::crop::attr::Crop;

#[derive(Debug)]
pub struct Farm<T: Crop> {
    pub crop: T,
}

impl<T: Crop> Farm<T> {
    pub fn new(crop: T) -> Farm<T> {
        Farm { crop }
    }
}