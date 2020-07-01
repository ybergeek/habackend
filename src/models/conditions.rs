use crate::config::DATE_FORMAT;
use chrono;
use diesel::sql_types::{Numeric, Text, Timestamptz};
use chrono::Utc;
use chrono::Local;
use chrono::prelude::*;

#[derive(Serialize)]
pub struct Conditions {
    pub time: chrono::NaiveDateTime,
    pub device_id: String,
    pub temperature: Option<bigdecimal::BigDecimal>,
    pub humidity: Option<bigdecimal::BigDecimal>,
}


#[derive(QueryableByName, Debug)]

pub struct AvgMinMax {
    #[sql_type = "Text"]
    pub device_id: String,
    #[sql_type = "Numeric"]
    pub avg_temp: bigdecimal::BigDecimal,
    #[sql_type = "Numeric"]
    pub min_temp: bigdecimal::BigDecimal,
    #[sql_type = "Text"]
    pub min_time: String,
    #[sql_type = "Numeric"]
    pub max_temp: bigdecimal::BigDecimal,
    #[sql_type = "Text"]
    pub max_time: String,
}
impl AvgMinMax {
    pub fn attach(self) -> AvgMinMaxJson {
        AvgMinMaxJson {
            device_id: self.device_id,
            avg_temp: self.avg_temp.to_string(),
            min_temp: self.min_temp.to_string(),
            min_time: self.min_time ,
            max_temp: self.max_temp.to_string(),
            max_time: self.max_time ,
        }
    }
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AvgMinMaxJson {
    pub device_id: String,
    pub avg_temp: String,
    pub min_temp: String,
    pub min_time: String,
    pub max_temp: String,
    pub max_time: String,
}

#[derive(QueryableByName, Queryable, Debug)]
#[table_name = "Conditions"]
pub struct FilterAvgMinMax {
    #[sql_type = "Timestamptz"]
    pub before: chrono::NaiveDateTime,
    #[sql_type = "Timestamptz"]
    pub after: chrono::NaiveDateTime,
}

#[derive(Queryable, Debug, Serialize)]
pub struct ConditionView {
    pub time: chrono:: DateTime<Utc> ,
    pub device_id: String,
    pub temperature: bigdecimal::BigDecimal,
}

impl ConditionView {
    pub fn attach(self) -> ConditionViewJson {
         ConditionViewJson {
            device_id: self.device_id,
            temperature: self.temperature.with_scale(3).to_string(),
            time: ConditionView::converted(self.time),
        }
    }
    fn converted(time: chrono::DateTime<Utc> ) -> String {
        let converted: DateTime<Local> = DateTime::from(time);
        converted.format(DATE_FORMAT).to_string()
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConditionViewJson {
    pub device_id: String,
    pub time: String,
    pub temperature: String,
}
