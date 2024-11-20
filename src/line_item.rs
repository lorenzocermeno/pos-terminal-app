pub struct LineItem {
    pub price: f32,
    pub quantity: u32,
}

impl LineItem {
    pub fn new(price: f32, quantity: u32) -> LineItem {
        LineItem {
            price,
            quantity,
        }
    }
}
