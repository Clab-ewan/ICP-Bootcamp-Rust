//    - Define an `Order` struct that associates users with purchased products
//    - Include order status using an enum
//    - Implement methods for creating, updating, and displaying orders
use crate::product::{Product};
use crate::user::{User};

pub struct Order {
    pub user: User,
    pub products: Vec<Product>,
    pub status: OrderStatus,
}

pub enum OrderStatus {
    Received,
    OnTheWay,
    Delivered,
}

impl Order {
    pub fn new(user: User, products: Vec<Product>) -> Order {
        Order {
            user,
            products,
            status: OrderStatus::Received,
        }
    }

    pub fn update(self: &mut Order, status: OrderStatus) {
        self.status = status;
    }

    pub fn display(self: &Order) {
        // Display user information
        self.user.display();
        
        // Display order status
        println!("Order status: {}", match self.status {
            OrderStatus::Received => "Received",
            OrderStatus::OnTheWay => "On the Way",
            OrderStatus::Delivered => "Delivered",
        });
        
        // Display products in order
        println!("Ordered products:");
        for product in &self.products {
            product.display();
        }
    }
}