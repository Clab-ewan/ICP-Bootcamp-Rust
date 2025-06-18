// - Define functions to add, remove, and check stock of products
// - Track product quantities

use crate::product::{Product};

pub struct Inventory {
    pub product_quantity: Vec<(Product, u16)>,
}

impl Inventory {   
    pub fn add(self: &mut Inventory, product: Product, quantity: u16) {
        self.product_quantity.push((product, quantity));
    }

    pub fn remove(self: &mut Inventory, product: &Product, quantity_to_remove: u16) {
        if let Some(item) = self.product_quantity.iter_mut().find(|(p, _)| p == product) {
            if item.1 >= quantity_to_remove {
            item.1 -= quantity_to_remove;
            }
        }
    }

    pub fn check(self: &mut Inventory, product: Product) -> u16 {
        if let Some(item) = self.product_quantity.iter().find(|(p, _)| p == &product) {
            return item.1;
        }
        return 0u16;
    }
}