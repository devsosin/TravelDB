use bigdecimal::BigDecimal;

pub struct City {
    pub id: i16,
    pub name: String,
    pub lat: BigDecimal,
    pub lng: BigDecimal,
    pub country_id: Option<i16>,
}
