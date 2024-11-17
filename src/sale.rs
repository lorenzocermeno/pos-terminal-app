use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct Sale {
    pub date: DateTime<Utc>,
    pub total: f32,
    pub id: String,
}
impl Sale { pub fn new(date: DateTime<Utc>, total: f32, id: String) -> Sale { Sale { date, total, id } } }