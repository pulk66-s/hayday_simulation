use std::fmt;

#[derive(Clone)]
pub struct Chicken {

}

impl fmt::Display for Chicken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Chicken")
    }
}
