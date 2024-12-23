impl Store {
    fn add_product(&mut self, name: String, description: String, price: f64, quantity: u32) {
        let product = Product {
            name: name.clone(),
            description,
            price,
            quantity,
        };
        self.inventory.insert(name, product);
    }

    fn edit_product(&mut self, name: &str, new_price: Option<f64>, new_quantity: Option<u32>) {
        if let Some(product) = self.inventory.get_mut(name) {
            if let Some(price) = new_price {
                product.price = price;
            }
            if let Some(quantity) = new_quantity {
                product.quantity = quantity;
            }
        } else {
            println!("Product not found.");
        }
    }

    fn delete_product(&mut self, name: &str) {
        if self.inventory.remove(name).is_some() {
            println!("Product removed.");
        } else {
            println!("Product not found.");
        }
    }
}
