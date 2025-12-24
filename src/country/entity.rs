use bigdecimal::BigDecimal;

pub struct Country {
    pub id: i16,
    pub name: String,
    pub lat: BigDecimal,
    pub lng: BigDecimal,
    pub continent_id: Option<i16>,
}
