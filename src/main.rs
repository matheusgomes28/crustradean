mod data_feed;

use data_feed::alpha_vantage::AlphaVantageDataFeed;
use data_feed::traits::DataFeed;

use anyhow::Result;
use clap::Parser;
use console::Emoji;

#[derive(Parser)]
struct ProgramArgs {
    #[clap(long, short)]
    pub alpha_vantage_key: String,
}

async fn try_main(program_args: ProgramArgs) -> Result<()> {

    let _ = env_logger::try_init()?;

    // Read the API key and pass to the feed
    let alpha_vantage_apikey = std::fs::read_to_string(program_args.alpha_vantage_key)?;

    let alpha_vantage_datafeed = AlphaVantageDataFeed::new(alpha_vantage_apikey);
    let prices = alpha_vantage_datafeed.update("IBM").await?;


    for update in prices {
        // Print the logs here
        println!("{} IBM", Emoji("📈", "[stock]"));
        println!(" - open: {}", update.open);
        println!(" - low: {}", update.low);
        println!(" - high: {}", update.high);
        println!(" - volume: {}", update.volume);
        println!();
    }
    
    Ok(())
}

#[tokio::main]
async fn main() {
    let program_args = ProgramArgs::parse();   

    match try_main(program_args).await {
        Err(e) => {
            panic!("Failed to run: {e}");
        }
        _ => {}
    }
}
