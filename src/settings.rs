use std::{io::Write, fs::OpenOptions, path::PathBuf};

use iced::Color;
use tokio::{io::{AsyncRead, AsyncReadExt}};

#[derive(Debug, Clone, PartialEq)]
pub struct Settings {
    pub source: PathBuf,
    pub show_all_coins: bool,
    pub show_all_currencies: bool,
    pub graph_color: Color
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            source: PathBuf::new(),
            show_all_coins: false,
            show_all_currencies: false,
            graph_color: Color::from_rgb8(0, 200, 0)
        }
    }
}

impl Settings {
    pub async fn read<R: AsyncRead + Unpin>(input: &mut R, source: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let show_all_coins = input.read_u8().await? == 1;
        let show_all_currencies = input.read_u8().await? == 1;
        let mut buff: [u8; 4] = [0; 4];
        input.read_exact(&mut buff).await?;
        let r = f32::from_ne_bytes(buff);
        input.read_exact(&mut buff).await?;
        let g = f32::from_ne_bytes(buff);
        input.read_exact(&mut buff).await?;
        let b = f32::from_ne_bytes(buff);
        input.read_exact(&mut buff).await?;
        let a = f32::from_ne_bytes(buff);
        let graph_color = Color {
            r,
            g,
            b,
            a,
        };
        Ok(Self {
            source,
            show_all_coins,
            show_all_currencies,
            graph_color
        })
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Source: {:?}", self.source);
        let mut file = OpenOptions::new().create(true).truncate(true).write(true).open(&self.source)?;
        file.write(&[self.show_all_coins as u8])?;
        file.write(&[self.show_all_currencies as u8])?;
        file.write(&self.graph_color.r.to_ne_bytes())?;
        file.write(&self.graph_color.g.to_ne_bytes())?;
        file.write(&self.graph_color.b.to_ne_bytes())?;
        file.write(&self.graph_color.a.to_ne_bytes())?;
        Ok(())
    }
}