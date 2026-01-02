use serde::{Serialize, Deserialize};
use crate::inputs::Side;

#[derive(Debug, Serialize, Deserialize)]
pub struct trade_order_response_output {
    pub order_id: u32,
    pub price: u32,
    pub quantity: u32,
    pub user_id: u32,
    pub side: Side,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct order_deleted_output {
    pub filled_quantity: u32,
    pub average_price: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct market_depth_output {
    pub symbol: String,
    pub market_price: u32,
    pub limit_price: u32,
    pub volume: bool,   // true if total volume > quantity
}
