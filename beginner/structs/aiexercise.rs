struct Product {
    name: String,
    price: f32,
    in_stock: bool,
    stock_quantity: u32, // Add stock quantity field
}

impl Product {
    fn new(name: String, price: f32, stock_quantity: u32) -> Product {
        Product {
            name,
            price,
            in_stock: stock_quantity > 0, // in_stock is now based on quantity
            stock_quantity,
        }
    }

    fn calculate_sale_tax(&self) -> f32 {
        self.price * 0.1
    }

    fn set_price(&mut self, price: f32) {
        self.price = price;
    }

    fn display_stock(&self) {
        println!(
            "Current stock of {}: {} units",
            self.name, self.stock_quantity
        );
        println!(
            "Item is {}",
            if self.in_stock {
                "in stock"
            } else {
                "out of stock"
            }
        );
    }

    fn buy(mut self) -> i32 {
        if self.stock_quantity > 0 {
            self.stock_quantity -= 1;
            self.in_stock = self.stock_quantity > 0;
            let receipt_number = 777;
            println!("{} was bought!", self.name);
            println!("Receipt number: #{}", receipt_number);
            println!("Remaining stock: {}", self.stock_quantity);
            receipt_number
        } else {
            println!("Sorry, {} is out of stock!", self.name);
            0 // Return 0 for failed purchase
        }
    }
}

fn main() {
    // Create a product with initial stock
    let mut book = Product::new(
        String::from("rust book"),
        13.37,
        5, // Initial stock of 5 books
    );

    // Display initial stock
    println!("\n--- Initial Stock ---");
    book.display_stock();

    // Make a purchase and store receipt number
    println!("\n--- Making Purchase ---");
    let receipt = book.buy();
    println!("Transaction complete with receipt #{}", receipt);

    // Create another product with low stock for demonstration
    let low_stock_book = Product::new(
        String::from("rare rust book"),
        99.99,
        1, // Only one in stock
    );

    println!("\n--- Low Stock Item ---");
    low_stock_book.display_stock();
    let receipt2 = low_stock_book.buy();

    // Try to buy the last book (should fail as it's now out of stock)
    // Note: We can't actually do this because the previous buy() took ownership
    // This is a good learning point about Rust ownership!
}
