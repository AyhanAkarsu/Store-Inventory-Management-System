impl Store {
    fn generate_report(&self) {
        println!("\n--- Inventory Report ---");
        for product in self.inventory.values() {
            println!(
                "Name: {}, Description: {}, Price: ${:.2}, Quantity: {}",
                product.name, product.description, product.price, product.quantity
            );
        }

        println!("\n--- Transaction History ---");
        for transaction in &self.transactions {
            match transaction {
                Transaction::Sale {
                    product_name,
                    quantity,
                    sale_price,
                } => {
                    println!(
                        "Sale - Product: {}, Quantity: {}, Sale Price: ${:.2}",
                        product_name, quantity, sale_price
                    );
                }
                Transaction::Purchase {
                    product_name,
                    quantity,
                    purchase_price,
                } => {
                    println!(
                        "Purchase - Product: {}, Quantity: {}, Purchase Price: ${:.2}",
                        product_name, quantity, purchase_price
                    );
                }
            }
        }

        println!("\nTotal Sales: ${:.2}, Total Profit: ${:.2}", self.total_sales, self.total_profit);
    }
}
