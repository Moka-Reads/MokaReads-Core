use crate::cpi::Data;
use once_cell::sync::Lazy;
use rust_decimal::prelude::{FromPrimitive, ToPrimitive};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::ops::Range;
use std::path::Path;
use tokio::fs::File;
use tokio::io::{AsyncWriteExt, Result};

/// Tier 0 represents the price of Free material
pub const TIER0: f64 = 0.0;
/// Tier 1 represents the cheapest range a product can be
pub const TIER1: Range<f64> = 9.99..35.99;
/// Tier 2 represents the median price of an ebook
pub const TIER2: Range<f64> = 35.99..40.99;
/// Tier 3 represents the minimum price range of a paperback
pub const TIER3: Range<f64> = 40.99..49.99;
/// Tier 4 represents the max price range of a paperback
pub const TIER4: Range<f64> = 49.99..69.99;
/// Tier 5 represents the price range of a hardcover
pub const TIER5: Range<f64> = 69.99..99.99;

static PRICES_PATH: Lazy<&Path> = Lazy::new(|| Path::new("./prices"));

/// Represents the price adjusted range given an annul year to calculate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Price {
    year: i32,
    tier1: Range<f64>,
    tier2: Range<f64>,
    tier3: Range<f64>,
    tier4: Range<f64>,
    tier5: Range<f64>,
}

trait Round {
    fn r2(self) -> f64;
}

impl Round for f64 {
    fn r2(self) -> f64 {
        let number = Decimal::from_f64(self).unwrap();
        number.round_dp(2).to_f64().unwrap()
    }
}

impl Price {
    pub async fn new(year: i32) -> Option<Self> {
        let data = Data::new().await;
        let rate = match data.calc_year(year) {
            Some(r_curr) => match data.calc_year(year - 1) {
                Some(r_old) => (r_curr - r_old) + 1.0,
                None => return None,
            },
            None => return None,
        };
        Some(Self {
            year,
            tier1: (TIER1.start * rate).r2()..(TIER1.end * rate).r2(),
            tier2: (TIER2.start * rate).r2()..(TIER2.end * rate).r2(),
            tier3: (TIER3.start * rate).r2()..(TIER3.end * rate).r2(),
            tier4: (TIER4.start * rate).r2()..(TIER4.end * rate).r2(),
            tier5: (TIER5.start * rate).r2()..(TIER5.end * rate).r2(),
        })
    }
    pub async fn save(&self) -> Result<()> {
        let path = PRICES_PATH.join(format!("{}.json", self.year));
        let json = serde_json::to_string_pretty(self).unwrap();
        let mut file = File::create(path).await?;
        file.write_all(json.as_bytes()).await?;
        Ok(())
    }
}
