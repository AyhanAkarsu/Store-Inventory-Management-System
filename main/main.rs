fn main() {
    let mut store = Store {
        inventory: HashMap::new(),
        transactions: vec![],
        total_sales: 0.0,
        total_profit: 0.0,
        authenticated: false,
    };

    println!("Welcome to the Rusty Store Inventory Management System.");
    println!("Please enter the password to continue:");
    
    let mut password = String::new();
    std::io::stdin().read_line(&mut password).unwrap();
    if !store.authenticate(password.trim()) {
        return;
    }

    loop {
        println!("\nMenu:");
        println!("1. Add Product");
        println!("2. Edit Product");
        println!("3. Delete Product");
        println!("4. Record Sale");
        println!("5. Record Purchase");
        println!("6. Generate Report");
        println!("7. Exit");

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();
        match choice.trim() {
            "1" => {
                let mut name = String::new();
                let mut description = String::new();
                let mut price = String::new();
                let mut quantity = String::new();

                println!("Enter product name:");
                std::io::stdin().read_line(&mut name).unwrap();

                println!("Enter product description:");
                std::io::stdin().read_line(&mut description).unwrap();

                println!("Enter product price:");
                std::io::stdin().read_line(&mut price).unwrap();

                println!("Enter product quantity:");
                std::io::stdin().read_line(&mut quantity).unwrap();

                let price: f64 = price.trim().parse().expect("Invalid price.");
                let quantity: u32 = quantity.trim().parse().expect("Invalid quantity.");
                store.add_product(name.trim().to_string(), description.trim().to_string(), price, quantity);
            }
            "2" => {
                let mut name = String::new();
                let mut new_price = String::new();
                let mut new_quantity = String::new();

                println!("Enter product name to edit:");
                std::io::stdin().read_line(&mut name).unwrap();

                println!("Enter new price (leave blank to skip):");
                std::io::stdin().read_line(&mut new_price).unwrap();

                println!("Enter new quantity (leave blank to skip):");
                std::io::stdin().read_line(&mut new_quantity).unwrap();

                let new_price = if new_price.trim().is_empty() {
                    None
                } else {
                    Some(new_price.trim().parse().expect("Invalid price."))
                };

                let new_quantity = if new_quantity.trim().is_empty() {
                    None
                } else {
                    Some(new_quantity.trim().parse().expect("Invalid quantity."))
                };

                store.edit_product(name.trim(), new_price, new_quantity);
            }
            "3" => {
                let mut name = String::new();
                println!("Enter product name to delete:");
                std::io::stdin().read_line(&mut name).unwrap();
                store.delete_product(name.trim());
            }
            "4" => {
                let mut name = String::new();
                let mut quantity = String::new();
                let mut price = String::new();

                println!("Enter product name to sell:");
                std::io::stdin().read_line(&mut name).unwrap();

                println!("Enter quantity:");
                std::io::stdin().read_line(&mut quantity).unwrap();

                println!("Enter sale price:");
                std::io::stdin().read_line(&mut price).unwrap();

                let quantity: u32 = quantity.trim().parse().expect("Invalid quantity.");
                let price: f64 = price.trim().parse().expect("Invalid price.");
                store.record_sale(name.trim(), quantity, price);
            }
            "5" => {
                let mut name = String::new();
                let mut quantity = String::new();
                let mut price = String::new();

                println!("Enter product name to purchase:");
                std::io::stdin().read_line(&mut name).unwrap();

                println!("Enter quantity:");
                std::io::stdin().read_line(&mut quantity).unwrap();

                println!("Enter purchase price:");
                std::io::stdin().read_line(&mut price).unwrap();

                let quantity: u32 = quantity.trim().parse().expect("Invalid quantity.");
                let price: f64 = price.trim().parse().expect("Invalid price.");
                store.record_purchase(name.trim(), quantity, price);
            }
            "6" => {
                store.generate_report();
            }
            "7" => {
                println!("Exiting system. Goodbye!");
                break;
            }
            _ => {
                println!("Invalid choice. Please try again.");
            }
        }
    }
}
