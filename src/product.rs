pub struct Product {
    pub price: f32,
    pub quantity: u32,
}

impl Product {
    pub fn new(price: f32, quantity: u32) -> Product {
        Product {
            price,
            quantity,
        }
    }
}
