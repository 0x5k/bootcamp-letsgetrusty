struct Product {
    name: String,
    price: f32,
    in_stock: bool,
}

impl Product {
    // Constructor with default values, associated function
    fn new(name: String, price: f32) -> Product {
        Product {
            name: name,
            price: price,
            in_stock: true,
        }
    }
    // Static methods, not associated with any instance
    fn default_tax() -> f32 {
        0.1
    }
    // mutable borrow self, associated function
    fn set_price(&mut self, price: f32) {
        self.price = price;
    }
    // immutable borrow self, associated function
    fn calculate_sale_tax(&self) -> f32 {
        self.price * Product::default_tax()
    }
    // own form of self, associated function
    fn buy(self) -> i32 {
        let name = self.name;
        println!("{name} was bougth!");
        777
    }
}
fn main() {
    let mut book = Product::new(String::from("rust book"), 1337.0);
    println!("normal price: {}", book.price);
    println!("{} is in stock: {}", book.name, book.in_stock);
    let sales_tax = book.calculate_sale_tax();
    println!("tax is {}", sales_tax);
    book.set_price(13333.7);
    let new_tax = book.calculate_sale_tax();
    println!("hacked price: {}", book.price);
    println!("new tax is {}", new_tax);
    let order_number = book.buy();
    println!("Order number: {}", order_number);
    // book.set_price(13333.7); - error because book is dropped and should not be used again
}

// constructor pattern
// Approach 1: Using Default trait for optional fields
// #[derive(Default)]
// struct Student {
//     first_name: String,
//     last_name: String,
//     roll_no: u16,
//     age: Option<u8>,         // Optional field
//     email: Option<String>,   // Optional field
//     address: Option<String>, // Optional field
// }

// impl Student {
//     // Basic constructor with required fields
//     fn new(first_name: String, last_name: String, roll_no: u16) -> Student {
//         Student {
//             first_name,
//             last_name,
//             roll_no,
//             ..Default::default() // Fill remaining fields with default values
//         }
//     }

//     // Method to set optional fields
//     fn with_age(mut self, age: u8) -> Self {
//         self.age = Some(age);
//         self
//     }

//     fn with_email(mut self, email: String) -> Self {
//         self.email = Some(email);
//         self
//     }

//     fn with_address(mut self, address: String) -> Self {
//         self.address = Some(address);
//         self
//     }
// }

// // Approach 2: Builder Pattern
// struct StudentBuilder {
//     first_name: String,
//     last_name: String,
//     roll_no: u16,
//     age: Option<u8>,
//     email: Option<String>,
//     address: Option<String>,
// }

// impl StudentBuilder {
//     fn new(first_name: String, last_name: String, roll_no: u16) -> Self {
//         StudentBuilder {
//             first_name,
//             last_name,
//             roll_no,
//             age: None,
//             email: None,
//             address: None,
//         }
//     }

//     fn age(mut self, age: u8) -> Self {
//         self.age = Some(age);
//         self
//     }

//     fn email(mut self, email: String) -> Self {
//         self.email = Some(email);
//         self
//     }

//     fn address(mut self, address: String) -> Self {
//         self.address = Some(address);
//         self
//     }

//     fn build(self) -> Student {
//         Student {
//             first_name: self.first_name,
//             last_name: self.last_name,
//             roll_no: self.roll_no,
//             age: self.age,
//             email: self.email,
//             address: self.address,
//         }
//     }
// }

// fn main() {
//     // Using Default approach
//     let student1 = Student::new("Harry".to_string(), "Potter".to_string(), 42)
//         .with_age(17)
//         .with_email("harry@hogwarts.com".to_string())
//         .with_address("4 Privet Drive".to_string());

//     // Using Builder Pattern
//     let student2 = StudentBuilder::new("Hermione".to_string(), "Granger".to_string(), 43)
//         .age(17)
//         .email("hermione@hogwarts.com".to_string())
//         .address("London".to_string())
//         .build();

//     println!("Student 1: {} {}", student1.first_name, student1.last_name);
//     println!("Student 2: {} {}", student2.first_name, student2.last_name);
// }
