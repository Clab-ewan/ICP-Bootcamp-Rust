// - Define a `User` struct with fields for id, name, email, and address
// - Implement methods for creating and displaying users

pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub address: String,
}

impl User {
    pub fn new(id: String, name: String, email: String, address: String) -> User {
        User {
            id,
            name,
            email,
            address,
        }
    }

    pub fn display(self: &User) {
        println!("User: {}, with ID: {} with email: {} is at {} ", self.name, self.id, self.email, self.address);
    }
}