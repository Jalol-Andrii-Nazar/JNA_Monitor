#![feature(total_cmp)]

use std::io::Cursor;

use iced::{Application, Settings, window::{self, Icon}};
use image::ImageFormat;

mod settings;
mod loading_gui;
mod gui;
mod main_tab_gui;
mod triggers_tab_gui;
mod settings_tab_gui;
mod about_tab_gui;

use std::{thread, time};
use notify_rust::Notification;

const ICON: &[u8] = include_bytes!("../icon.png");

pub const NAME: &'static str = "JNA";
pub const VERSION: &'static str = "alpha-0.1";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
      
    let mut iced_settings: Settings<()> = Default::default();
    let mut iced_wsettings: window::Settings = window::Settings::default();

    let rgba = image::io::Reader::with_format(Cursor::new(ICON), ImageFormat::Png)
        .decode()?
        .to_rgba8();
    let icon_width = rgba.width();
    let icon_height = rgba.height();

    iced_wsettings.icon = Some(Icon::from_rgba(rgba.into_raw(), icon_width, icon_height).unwrap());
    iced_wsettings.size = (1024, 720);
    iced_wsettings.resizable = false;
    iced_settings.window = iced_wsettings;

    loading_gui::Gui::run(iced_settings)?;
    
  /*
    while true {
        println!("Checking!");
        check_triggers().await?;
        let ten_millis = time::Duration::from_secs(60);
        thread::sleep(ten_millis);
    }*/
    Ok(())
    
}

pub async fn check_triggers() -> Result<(), Box<dyn std::error::Error>> {
    let api_client = coingecko_requests::api_client::Client::new();
    let mut client = coingecko_requests::caching_client::Client::new(api_client).await?;
    let triggers = client.get_all_triggers().await?;

    for trigger in triggers {
        let mut increase= true;

        if trigger.old_price > trigger.new_price {
            increase = false;
        }

        let coin_owned = vec![String::from(&trigger.coin)];
        let coin_half_owned: Vec<_> = coin_owned.iter().map(String::as_str).collect();
    
        let currency_owned = vec![String::from(&trigger.currency)];
        let currency_half_owned: Vec<_> = currency_owned.iter().map(String::as_str).collect();
    
        let price = client.price(&coin_half_owned, &currency_half_owned).await?;
        let price = price[&trigger.coin][&trigger.currency];

        if (increase && price >= trigger.new_price) || (!increase && price <= trigger.new_price){
            //println!("{} => {}\nOld Price: {}\nNew Price: {}\nChanged: {}", trigger.coin.to_uppercase(), trigger.currency.to_uppercase(), trigger.old_price, price, (price - trigger.old_price as f64).abs());
        
            Notification::new()
                .appname("JNA Monitor")
                .summary(&format!("{} => {}", trigger.coin.to_uppercase(), trigger.currency.to_uppercase()))
                .body(&format!("Old Price: {}\nCurrent Price: {}\nDifference: {}", trigger.old_price as i64, price, (price - trigger.old_price).abs() as i64))
                .icon("D:/Projects/Organisation/mywork/JNA_Monitor/icon.png")
                .show()?;
        } 

    }

    Ok(())
}