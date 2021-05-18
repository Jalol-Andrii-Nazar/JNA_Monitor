use std::{rc::Rc, sync::{Arc, RwLock}};

use directories::ProjectDirs;
use iced::{Application, Clipboard, Column, Command, Text, executor};
use tokio::fs::OpenOptions;

enum State {
    Initilizing,
    Errored,
    Initialized(crate::gui::Gui)
}

#[derive(Debug)]
pub enum Message {
    SettingsLoaded(crate::settings::Settings),
    CoinsLoaded(Vec<coingecko_requests::data::Coin>),
    CurrenciesLoaded(Vec<coingecko_requests::data::VsCurrency>),
    Error(String),
    GuiMessage(crate::gui::Message)
}

pub struct Gui {
    messages: Vec<String>,
    state: State,
    settings: Option<crate::settings::Settings>,
    coins: Option<Vec<coingecko_requests::data::Coin>>,
    currencies: Option<Vec<coingecko_requests::data::VsCurrency>>,
}

impl Application for Gui {
    type Executor = executor::Default;

    type Message = Message;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let messages = vec![format!("Loading '{}' v. {}. Please wait...", crate::NAME, crate::VERSION), format!("Loading settings...")];
        (Self {
            messages,
            state: State::Initilizing,
            settings: None,
            coins: None,
            currencies: None,
        }, Command::perform(load_settings(), unwrap_result))
    }

    fn title(&self) -> String {
        if let State::Initialized(ref _gui) = self.state {
            format!("{} v.{}", crate::NAME, crate::VERSION)
        } else {
            format!("Loading...")
        }
    }

    fn update(&mut self, message: Self::Message, clipboard: &mut Clipboard) -> Command<Self::Message> {
        match message {
            Message::SettingsLoaded(settings) => {
                self.settings = Some(settings);
                self.messages.push(format!("Settings loaded successfully!"));
                self.messages.push(format!("Loading coins..."));
                Command::perform(load_coins(), unwrap_result)
            }
            Message::CoinsLoaded(coins) => {
                self.coins = Some(coins);
                self.messages.push(format!("Coins loaded successfully..."));
                self.messages.push(format!("Loading currencies..."));
                Command::perform(load_vs_currencies(), unwrap_result)
            }
            Message::CurrenciesLoaded(vs_currencies) => {
                self.currencies = Some(vs_currencies);
                self.messages.push(format!("Currencies loaded successfully..."));
                self.messages.push(format!("Starting the GUI..."));
                let coins = Rc::new(self.coins.take().unwrap());
                let currencies = Rc::new(self.currencies.take().unwrap());
                let settings = Arc::new(RwLock::new(self.settings.take().unwrap()));
                let (gui, gui_message) = crate::gui::Gui::new(crate::gui::Flags {
                    coins,
                    currencies,
                    settings,
                });
                self.state = State::Initialized(gui);
                gui_message.map(Self::Message::GuiMessage)
            }
            Message::Error(error) => {
                self.state = State::Errored;
                self.messages.push(format!("An error happened! {}", error));
                Command::none()
            }
            Message::GuiMessage(msg) => {
                if let State::Initialized(ref mut gui) = self.state {
                    gui.update(msg, clipboard).map(Self::Message::GuiMessage)
                } else {
                    panic!("SHOULD NOT HAPPEN: gui message cannot be received before initialization is completed.")
                }
            }
        }
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        if let State::Initialized(ref mut gui) = self.state {
            gui.view().map(Message::GuiMessage)
        } else {
            let mut column = Column::new();
            for message in self.messages.iter() {
                column = column.push(Text::new(message));
            }
            column.into()
        }
    }
}

fn unwrap_result(result: Result<Message, Box<dyn std::error::Error>>) -> Message {
    match result {
        Ok(message) => { message }
        Err(err) => { Message::Error(err.to_string()) }
    }
}

async fn load_settings() -> Result<Message, Box<dyn std::error::Error>> {
    let project_dirs = ProjectDirs::from("org", "jna", "jna")
        .ok_or::<Box<dyn std::error::Error>>(From::from("Failed to get project_dirs!"))?;
    
    let config_dir = project_dirs.config_dir().to_owned();
    if !config_dir.exists() {
        tokio::fs::create_dir_all(&config_dir).await?;
    }
    let mut config_file = config_dir;
    config_file.set_file_name("config");
    config_file.set_extension("bin");
    if config_file.exists() {
        let mut file = OpenOptions::new().read(true).open(&config_file).await?;
        let settings = crate::settings::Settings::read(&mut file, config_file).await?;
        Ok(Message::SettingsLoaded(settings))
    } else {
        let settings = crate::settings::Settings {
            source: config_file,
            ..Default::default()
        };
        Ok(Message::SettingsLoaded(settings))
    }
}

async fn load_coins() -> Result<Message, Box<dyn std::error::Error>> {
    let api_client = coingecko_requests::api_client::Client::new();
    let mut caching_client = coingecko_requests::caching_client::Client::new(api_client).await?;
    let coins = caching_client.coins().await?;
    Ok(Message::CoinsLoaded(coins))
}

async fn load_vs_currencies() -> Result<Message, Box<dyn std::error::Error>> {
    let api_client = coingecko_requests::api_client::Client::new();
    let mut caching_client = coingecko_requests::caching_client::Client::new(api_client).await?;
    let currencies = caching_client.vs_currencies().await?;
    Ok(Message::CurrenciesLoaded(currencies))
}
