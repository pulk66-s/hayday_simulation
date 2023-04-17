use crate::types::Pos;
use crate::objects::build::Building;
use crate::game::context::Context;
use std::fmt;
use std::collections::HashMap;

use super::types::BarnContent;

#[derive(Clone, Debug)]
pub struct Barn {
    pub price: u32,
    pub size: Pos,
    pub capacity: u32,
    pub content: HashMap<BarnContent, u32>,
    pub capacity_used: u32,
    pub name: String,
}

impl Barn {
    pub fn new() -> Barn {
        Barn {
            price: 0,
            size: Pos { x: 1, y: 1 },
            capacity: 50,
            content: HashMap::new(),
            capacity_used: 0,
            name: "Barn".to_string(),
        }
    }

    pub fn add(&mut self, elem: BarnContent, amount: u32) -> bool {
        if self.capacity - self.capacity_used < amount {
            return false;
        }
        self.capacity_used += amount;
        if self.content.contains_key(&elem) {
            let current_amount = self.content.get(&elem).unwrap();
            self.content.insert(elem, current_amount + amount);
        } else {
            self.content.insert(elem, amount);
            println!("Created to barn");
        }
        println!("barn content: {:?}", self.content);
        return true;
    }

    pub fn remove(&mut self, elem: BarnContent, amount: u32) -> bool {
        if self.content.contains_key(&elem) {
            let current_amount = self.content.get(&elem).unwrap();
            if current_amount >= &amount {
                self.capacity_used -= amount;
                self.content.insert(elem, current_amount - amount);
                return true;
            }
        }
        return false;
    }

    pub fn have(&self, elem: BarnContent, amount: u32) -> bool {
        if self.content.contains_key(&elem) {
            let current_amount = self.content.get(&elem).unwrap();
            if current_amount >= &amount {
                return true;
            }
        }
        return false;
    }
}

impl Building for Barn {
    fn build(&self, ctx: &mut Context) -> bool {
        if ctx.player.money < self.price {
            return false;
        }
        ctx.player.money -= self.price;
        return true;
    }

    fn name(&self) -> String {
        return self.name.clone();
    }
}

impl fmt::Display for Barn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut content = format!("Barn: {{ price: {}, size: {}, capacity: {}, capacity_used: {}, content: {{", self.price, self.size, self.capacity, self.capacity_used);

        for (barn_content, amount) in &self.content {
            content += &format!("{}: {}, ", barn_content, amount);
        }
        content += "} }   ";
        write!(f, "{}", content)
    }
}

