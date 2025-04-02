struct Product {
    name: String,
    price: f32,
    in_stock: bool,
}

impl Product {
    fn new(name: String, price: f32) -> Product {
        Product {
            name: name,
            price: price,
            in_stock: true,
        }
    }
    fn default_tax() -> f32 {
        0.1
    }
    fn set_price(&mut self, price: f32) {
        self.price = price;
    }
    fn calculate_sale_tax(&self) -> f32 {
        self.price * Product::default_tax()
    }
    fn buy(self) -> i32 {
        let name = self.name;
        println!("{name} was bougth!");
        777
    }
}
fn main() {
    let mut book: Product = Product::new(String::from("rust book"), 1337.0);
    println!("normal price: {}", book.price);
    let sales_tax = book.calculate_sale_tax();
    println!("tax is {}", sales_tax);
    book.set_price(13333.7);
    println!("hacked price: {}", book.price);
    book.buy();
}
