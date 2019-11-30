
use chrono;




#[derive(Serialize)]
pub struct Conditions {
    pub time: chrono::NaiveDateTime,
    pub device_id: String,
    pub temperature: Option<bigdecimal::BigDecimal>,
    pub humidity: Option<bigdecimal::BigDecimal>,
}
