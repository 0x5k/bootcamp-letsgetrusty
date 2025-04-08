// Make the following code compile.

struct ShopItem {
    name: String,
    quantity: u32,
    in_stock: bool,
}

fn main() {
    let mut item = ShopItem {
        name: String::from("Socks"),
        quantity: 200,
        in_stock: true,
    };
    // 50 pairs of socks were sold
    item.quantity -= 50;
    if item.quantity == 0 {
        item.in_stock = false;
    }
    println!("{} is in stock: {}", item.name, item.in_stock);
}

//

struct ShopItem {
    name: String,
    quantity: u32,
}

impl ShopItem {
    fn new(name: String, quantity: u32) -> ShopItem {
        ShopItem { name, quantity }
    }
    fn in_stock(&self) -> bool {
        self.quantity > 0
    }
}

fn main() {
    let item = ShopItem::new("Pants".to_string(), 450);
    if item.in_stock() {
        println!("{} remaining: {}", item.name, item.quantity);
    } else {
        println!("{} not in stock", item.name);
    }
}

// struct ShopItem {
//     name: String,
//     quantity: u32,
// }

// fn main() {
//     let item = create_item("Socks", 200);
//     let in_stock = is_in_stock(&item);
//     println!("{} is in stock: {in_stock}", item.name);
// }

// fn create_item(name: &str, quantity: u32) -> ShopItem {
//     ShopItem {
//         name: name.to_string(),
//         quantity,
//     }
// }

// fn is_in_stock(item: &ShopItem) -> bool {
//     item.quantity > 0
// }
