use serde::{Serialize, Deserialize};
use crate::inputs::Side;

#[derive(Debug, Serialize, Deserialize)] // This is where serde is used to serialize and deserialize the response body
// This is the equivalent of the response body in the request body 
pub struct trade_order_response_output {
    pub order_id: u32,
    pub price: u32,
    pub quantity: u32,
    pub user_id: u32,
    pub side: Side
}