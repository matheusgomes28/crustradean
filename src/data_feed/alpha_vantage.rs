use anyhow::Result;
use chrono::TimeZone;
use serde::{de, Deserialize, Deserializer};
use serde_json::Value;
use std::collections::HashMap;


use super::{traits::DataFeed, PricePoint};

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct AlphaVantageMetadata {
    #[serde(rename = "1. Information")]
    pub information: String,
    #[serde(rename = "2. Symbol")]
    pub symbol: String,
    #[serde(rename = "3. Last Refreshed")]
    pub last_refreshed: String,
    #[serde(rename = "4. Interval")]
    pub interval: String,
    #[serde(rename = "5. Output Size")]
    pub output_size: String,
    #[serde(rename = "6. Time Zone")]
    pub time_zone: String,
}

#[derive(Debug, Deserialize)]
struct AlphaVantageIndexUpdate{
    #[serde(alias = "1. open", deserialize_with = "de_f64_or_string_as_f64")]
    pub open: f64,

    #[serde(alias = "2. high", deserialize_with = "de_f64_or_string_as_f64")]
    pub high: f64,

    #[serde(alias = "3. low", deserialize_with = "de_f64_or_string_as_f64")]
    pub low: f64,

    #[serde(alias = "4. close", deserialize_with = "de_f64_or_string_as_f64")]
    pub close: f64,

    #[serde(alias = "5. volume", deserialize_with = "de_i64_or_string_as_i64")]
    pub volume: i64,
}


#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct AlphaVantageResponse {
    #[serde(rename = "Meta Data")]
    pub meta_data: AlphaVantageMetadata,

    // TODO : Maybe these should be optional
    // #[serde(rename = "Time Series (1min)")]
    // pub timeseries_1min: HashMap<String, AlphaVantageIndexUpdate>,

    #[serde(rename = "Time Series (5min)")]
    pub timeseries_5min: HashMap<String, AlphaVantageIndexUpdate>,

    // TODO : Maybe these should be optional
    // #[serde(rename = "Time Series (15min)")]
    // pub timeseries_15min: HashMap<String, AlphaVantageIndexUpdate>,
}

/// Struct that implements a datafeed for the AlphaVantage
/// endpoints.
pub struct AlphaVantageDataFeed {
    /// API key to access the data
    pub apikey: String,
}

impl AlphaVantageDataFeed {
    pub fn new(apikey: String) -> Self {
        AlphaVantageDataFeed{
            apikey
        }
    }
}

impl DataFeed for AlphaVantageDataFeed {
    async fn update(&self, index: &str) -> Result<Vec<PricePoint>> {
        // This where we call the REST api for an index and parse it
        let url = "https://www.alphavantage.co/query?function=TIME_SERIES_INTRADAY&symbol=%index%&interval=5min&apikey=demo"
            .replace("%index%", index);

        let response = reqwest::get(url)
            .await?
            .json::<AlphaVantageResponse>()
            .await?;

        let timezone: chrono_tz::Tz = response.meta_data.time_zone.parse()?;

        // Convert all the retrieved values into Vec<PricePoint>
        
        let mut prices = Vec::<PricePoint>::new();
        for (timestamp_str, index_values) in response.timeseries_5min {

            // TODO : Ignore the data if we fail to parse the timestamp
            let timestamp = chrono::NaiveDateTime::parse_from_str(&timestamp_str, "%Y-%m-%d %H:%M:%S")?;
            let timestamp = timezone.from_local_datetime(&timestamp);

            if let chrono::LocalResult::Single(timestamp) = timestamp {
                prices.push(PricePoint{
                    open: index_values.open,
                    high: index_values.high,
                    low: index_values.low,
                    close: index_values.close,
                    volume: index_values.volume,
                    timestamp
                });
            } else {
                eprintln!("Datetime was ambiguous: {:#?}", timestamp);
            }
        }

        Ok(prices)
    }
}

/// This function is used to convert String values to f64 values
/// with serde's deserialisation
fn de_f64_or_string_as_f64<'de, D: Deserializer<'de>>(deserializer: D) -> Result<f64, D::Error> {
  Ok(match Value::deserialize(deserializer)? {
    Value::String(s) => s.parse().map_err(de::Error::custom)?,
    Value::Number(num) => num.as_f64().ok_or_else(|| de::Error::custom("Invalid number"))?,
    _ => return Err(de::Error::custom("wrong type")),
  })
}

/// This function is used to convert String values to i64 values
/// with serde's deserialisation
fn de_i64_or_string_as_i64<'de, D: Deserializer<'de>>(deserializer: D) -> Result<i64, D::Error> {
  Ok(match Value::deserialize(deserializer)? {
    Value::String(s) => s.parse().map_err(de::Error::custom)?,
    Value::Number(num) => num.as_i64().ok_or_else(|| de::Error::custom("Invalid number"))?,
    _ => return Err(de::Error::custom("wrong type")),
  })
}
