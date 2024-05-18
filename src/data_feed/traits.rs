use anyhow::Result;
use super::PricePoint;

pub trait DataFeed {
    fn update(&self, index: &str) -> impl std::future::Future<Output = Result<Vec<PricePoint>>> + Send;
}
