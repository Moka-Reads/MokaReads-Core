use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Observation {
    d: String,
    STATIC_INFLATIONCALC: StaticInflationCalcValue,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct StaticInflationCalcValue {
    v: String,
}

impl Observation {
    pub fn date(&self) -> NaiveDate {
        NaiveDate::parse_from_str(&self.d, "%Y-%m-%d").unwrap()
    }
    pub fn value(&self) -> f64 {
        let val = &self.STATIC_INFLATIONCALC.v;
        val.parse().unwrap()
    }
}
