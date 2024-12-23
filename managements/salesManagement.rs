impl Store {
    fn record_sale(&mut self, product_name: &str, quantity: u32, sale_price: f64) {
        if let Some(product) = self.inventory.get_mut(product_name) {
            if product.quantity >= quantity {
                product.quantity -= quantity;
                self.total_sales += sale_price * quantity as f64;
                self.total_profit += (sale_price - product.price) * quantity as f64;
                self.transactions.push(Transaction::Sale {
                    product_name: product_name.to_string(),
                    quantity,
                    sale_price,
                });
            } else {
                println!("Insufficient stock.");
            }
        } else {
            println!("Product not found.");
        }
    }
}
