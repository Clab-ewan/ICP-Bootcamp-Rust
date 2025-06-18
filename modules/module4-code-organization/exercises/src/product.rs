// Define a `Product` &Stringuct with fields for id, name, price, and description
// Implement methods for creating and displaying products

#[derive(PartialEq, Clone)]
pub struct Product {
    id: String,
    name: String,
    price: u16,
    description: String,
}

impl Product {
    pub fn new(id: String, name: String, price: u16, description: String) -> Product {
        Product {
            id,
            name,
            price,
            description,
        }
    }

    pub fn display(self: &Product) {
        println!("Product: {}, with ID: {} is {} and costs ${} ", self.name, self.id, self.description, self.price);
    }
}