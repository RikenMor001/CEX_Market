use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Side{
    Buy,
    Sell
}

#[derive(Debug, Serialize, Deserialize)]
pub struct trade_orders_input { // Create a struct to represent the request body
    pub price: u32,
    pub quantity: u32,
    pub user_id: u32,
    pub side: Side
}