mod observation;

use chrono::NaiveDate;
use observation::Observation;
use serde::{Deserialize, Serialize};

const ADDRESS: &str = "https://www.bankofcanada.ca/valet/observations/STATIC_INFLATIONCALC/json";

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Data {
    terms: Terms,
    seriesDetail: SeriesDetail,
    observations: Vec<Observation>,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Terms {
    url: String,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SeriesDetail {
    STATIC_INFLATIONCALC: StaticInflationCalc,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct StaticInflationCalc {
    label: String,
    description: String,
    dimension: Dimension,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Dimension {
    key: String,
    name: String,
}

impl Data {
    pub async fn new() -> Self {
        reqwest::get(ADDRESS)
            .await
            .expect("Couldn't retrieve data from Bank of Canada")
            .json()
            .await
            .expect("Couldn't deserialize data!")
    }
    pub fn latest_observation(&self) -> &Observation {
        &self.observations[self.observations.len() - 1]
    }
    pub fn latest_rate(&self) -> f64 {
        let latest_observation = self.latest_observation();
        latest_observation.value()
    }
    pub fn latest_date(&self) -> NaiveDate {
        let latest_observation = self.latest_observation();
        latest_observation.date()
    }
    pub fn find_observation(&self, date: NaiveDate) -> Option<&Observation> {
        let observation = self.observations.iter().find(|x| x.date() == date);
        observation
    }
    /// `from` represents the older date
    /// `to` represents the newer date
    pub fn calc(&self, from: NaiveDate, to: NaiveDate) -> Option<f64> {
        let from_observation = match self.find_observation(from) {
            Some(o) => o,
            None => return None,
        };
        let to_observation = match self.find_observation(to) {
            Some(o) => o,
            None => return None,
        };

        let from_value = from_observation.value();
        let to_value = to_observation.value();
        let val = (to_value - from_value) / from_value;
        Some(val)
    }
    pub fn calc_to_percent(&self, from: NaiveDate, to: NaiveDate) -> Option<f64> {
        match self.calc(from, to) {
            Some(v) => Some(v * 100.0),
            None => None,
        }
    }
    pub fn calc_year(&self, year: i32) -> Option<f64> {
        let from = match NaiveDate::from_ymd_opt(year, 1, 1) {
            Some(d) => d,
            None => return None,
        };
        let to = match NaiveDate::from_ymd_opt(year, 12, 1) {
            Some(d) => d,
            None => self.latest_date(),
        };
        match self.calc(from, to) {
            None => None,
            Some(val) => Some(val),
        }
    }
}
