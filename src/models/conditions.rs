
use chrono;




#[derive(Serialize)]
pub struct Conditions {
    pub time: chrono::NaiveDateTime,
    pub device_id: String,
    pub temperature: Option<bigdecimal::BigDecimal>,
    pub humidity: Option<bigdecimal::BigDecimal>,
}

#[derive(Serialize)]
pub struct AvgMinMax{
    pub hour: chrono::NaiveDateTime,
    pub avg: bigdecimal::BigDecimal,
    pub min: bigdecimal::BigDecimal,
    pub device_id: String,
}
