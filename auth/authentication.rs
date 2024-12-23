impl Store {
    fn authenticate(&mut self, password: &str) -> bool {
        if password == "admin123" {
            self.authenticated = true;
            println!("Authentication successful.");
        } else {
            println!("Authentication failed.");
        }
        self.authenticated
    }
}
