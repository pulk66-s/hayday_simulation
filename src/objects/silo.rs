use crate::objects::{
    build::Building,
    types::SiloContent,
};
use crate::types::Pos;
use crate::game::context::Context;
use std::fmt;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Silo {
    pub price: u32,
    pub size: Pos,
    pub capacity: u32,
    pub content: HashMap<SiloContent, u32>,
    pub capacity_used: u32,
    pub name: String,
}

impl Silo {
    pub fn new() -> Silo {
        Silo {
            price: 0,
            size: Pos { x: 1, y: 1 },
            capacity: 50,
            content: HashMap::new(),
            capacity_used: 0,
            name: "Silo".to_string(),
        }
    }

    pub fn add(&mut self, elem: SiloContent, amount: u32) -> bool {
        if self.capacity - self.capacity_used < amount {
            return false;
        }
        self.capacity_used += amount;
        if self.content.contains_key(&elem) {
            let current_amount = self.content.get(&elem).unwrap();
            self.content.insert(elem, current_amount + amount);
        } else {
            self.content.insert(elem, amount);
            println!("Created to silo");
        }
        println!("silo content: {:?}", self.content);
        return true;
    }
}

impl Building for Silo {
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

impl fmt::Display for Silo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut content = format!("Silo: {{ price: {}, size: {}, capacity: {}, capacity_used: {}, content: {{", self.price, self.size, self.capacity, self.capacity_used);

        for (silo_content, amount) in &self.content {
            content += &format!("{}: {}, ", silo_content, amount);
        }
        content += "} }   ";
        write!(f, "{}", content)
    }
}