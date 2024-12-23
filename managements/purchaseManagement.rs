impl Store {
    fn record_purchase(&mut self, product_name: &str, quantity: u32, purchase_price: f64) {
        if let Some(product) = self.inventory.get_mut(product_name) {
            product.quantity += quantity;
        } else {
            println!("Product not found. Adding it to inventory.");
            self.add_product(
                product_name.to_string(),
                "No description".to_string(),
                purchase_price,
                quantity,
            );
        }

        self.transactions.push(Transaction::Purchase {
            product_name: product_name.to_string(),
            quantity,
            purchase_price,
        });
    }
}
