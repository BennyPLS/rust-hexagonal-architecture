pub struct Order {
    pub field: String,
    pub order_type: OrderType
}

pub enum OrderType {
    ASC,
    DES
}
