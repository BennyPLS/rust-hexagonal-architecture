pub struct Filter {
    pub field: String,
    pub operator: Operator,
    pub value: String,
}

impl Filter {
    fn new(field: String, operator: Operator, value: String) -> Filter {
        Filter {
            field,
            operator,
            value
        }
    }
        
}

pub enum Operator {
    Equal,
    Greater,
    GreaterOrEqual,
    Less,
    LessOrEqual,
    Contains,
    NotContains,
}
