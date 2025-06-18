// TODO: Define your module structure here
// Hint: You'll need to create modules for products, users, orders, and inventory

pub mod product;
pub mod user;
pub mod order;
pub mod inventory;

use crate::product::Product;
use crate::inventory::Inventory;
use crate::user::User;
use crate::order::Order;

fn main() {
    // TODO: Use your modules to create a simple e-commerce workflow:
    // 1. Create some products
    let bread: Product = Product::new("0".to_string(), "bread".to_string(), 10, "Some piece of food".to_string());
    let meat: Product = Product::new("1".to_string(), "meat".to_string(), 30, "Some piece of food".to_string());
    // 2. Add products to inventory
    let mut inventory: Inventory = Inventory {
        product_quantity: Vec::new(),
    };
    inventory.add(bread.clone(), 10);
    inventory.add(meat.clone(), 30);
    // 3. Create a user
    let user1: User = User::new("1".to_string(), "user1".to_string(), "user1@gmail.com".to_string(), "10 avenue rust".to_string());
    // 4. Create an order for the user with some products
    let order: Order = Order::new(user1, vec![bread, meat]);
    // 5. Print order details
    order.display();
}