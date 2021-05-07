#![feature(total_cmp)]

use std::io::Cursor;

use iced::{Application, Settings, window::{self, Icon}};
use image::ImageFormat;

mod loading_gui;
mod gui;
mod main_tab_gui;
mod triggers_tab_gui;
mod settings_tab_gui;
mod about_tab_gui;

const ICON: &[u8] = include_bytes!("../icon.png");

pub const NAME: &'static str = "JNA";
pub const VERSION: &'static str = "alpha-0.1";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut settings: Settings<()> = Default::default();
    let mut wsettings: window::Settings = window::Settings::default();

    let rgba = image::io::Reader::with_format(Cursor::new(ICON), ImageFormat::Png)
        .decode()?
        .to_rgba8();
    let icon_width = rgba.width();
    let icon_height = rgba.height();

    wsettings.icon = Some(Icon::from_rgba(rgba.into_raw(), icon_width, icon_height).unwrap());
    wsettings.size = (1024, 720);
    wsettings.resizable = false;
    settings.window = wsettings;

    loading_gui::Gui::run(settings)?;

    Ok(())
}
