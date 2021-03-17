use iced::Application;

mod gui;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    let cg_client = coingecko_requests::client::Client::new();
    let settings = iced::Settings::with_flags(gui::GuiFlags {
        ids: cg_client.coins_list().await?.into_iter().map(|coin| coin.id).collect(),
        vs_currencies: cg_client.simple_supported_vs_currencies().await?
    });
    gui::Gui::run(settings)?;
    Ok(())
}
