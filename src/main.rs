use chrono::NaiveDateTime;
use iced::Application;

mod gui;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    let cg_client = coingecko_requests::client::Client::new();
    let btc_to_usd = cg_client.coins_id_market_chart_range("bitcoin", "usd", 1392577232, 1422577232)
        .await?
        .prices
        .into_iter()
        .map(|(timestamp, price)| (NaiveDateTime::from_timestamp(timestamp as i64 / 1000, 0), price))
        .collect::<Vec<_>>();
    let mut settings = iced::Settings::with_flags(gui::GuiFlags {
        ids: cg_client.coins_list().await?.into_iter().map(|coin| coin.id).collect(),
        vs_currencies: cg_client.simple_supported_vs_currencies().await?,
        btc_to_usd
    });

    settings.window = iced::window::Settings {
        resizable: false,
        ..Default::default()
    };

    gui::Gui::run(settings)?;
    Ok(())
}
