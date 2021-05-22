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
mod styling;

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

    Ok(())
    
}
