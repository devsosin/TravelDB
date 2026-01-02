use bigdecimal::BigDecimal;

pub struct Country {
    pub id: i16,
    pub name: String,
    pub lat: BigDecimal,
    pub lng: BigDecimal,
}

impl Country {
    pub fn new(name: &str) -> Self {
        Self {
            id: 0,
            name: name.into(),
            lat: BigDecimal::default(),
            lng: BigDecimal::default(),
        }
    }
}
