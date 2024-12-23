use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Product {
    name: String,
    description: String,
    price: f64,
    quantity: u32,
}

#[derive(Debug)]
enum Transaction {
    Sale {
        product_name: String,
        quantity: u32,
        sale_price: f64,
    },
    Purchase {
        product_name: String,
        quantity: u32,
        purchase_price: f64,
    },
}

#[derive(Debug)]
struct Store {
    inventory: HashMap<String, Product>,
    transactions: Vec<Transaction>,
    total_sales: f64,
    total_profit: f64,
    authenticated: bool,
}
