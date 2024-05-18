pub struct PricePoint {
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: i64,
    pub timestamp: chrono::DateTime<chrono_tz::Tz>,
}
