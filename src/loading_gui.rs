use std::{rc::Rc, sync::{Arc, RwLock}};

use directories::ProjectDirs;
use iced::{Application, Clipboard, Column, Command, Text, executor};
use tokio::fs::OpenOptions;

use notify_rust::{Notification};
use std::{time, thread};

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

    tokio::spawn(async move {
        spawn_check_triggers().await.unwrap();
    });

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
    let caching_client = coingecko_requests::caching_client::Client::new(api_client).await?;
    let coins = caching_client.coins().await?;
    Ok(Message::CoinsLoaded(coins))
}

async fn load_vs_currencies() -> Result<Message, Box<dyn std::error::Error>> {
    let api_client = coingecko_requests::api_client::Client::new();
    let caching_client = coingecko_requests::caching_client::Client::new(api_client).await?;
    let currencies = caching_client.vs_currencies().await?;
    Ok(Message::CurrenciesLoaded(currencies))
}

async fn spawn_check_triggers() -> Result<(), Box<dyn std::error::Error>> {
    let api_client = coingecko_requests::api_client::Client::new();
    let caching_client = coingecko_requests::caching_client::Client::new(api_client).await?;
    let coins = caching_client.coins().await?;
    let currencies = caching_client.vs_currencies().await?;

    loop {
        println!("Checking");
        check_triggers(coins.clone(), currencies.clone()).await.unwrap();
        thread::sleep(time::Duration::from_secs(60));
    }
}

async fn check_triggers(coins: Vec<coingecko_requests::data::Coin>, currencies: Vec<coingecko_requests::data::VsCurrency>) -> Result<(), Box<dyn std::error::Error>> {
    let api_client = coingecko_requests::api_client::Client::new();
    let client = coingecko_requests::caching_client::Client::new(api_client).await?;
    let triggers = client.get_all_triggers().await?;
    for trigger in triggers {
        let mut increase= true;
        let coin = coins.iter().find(|coin| coin.rowid == trigger.coin_id).cloned().unwrap();
        let currency = currencies.iter().find(|currency| currency.rowid == trigger.currency_id).cloned().unwrap();
        
        if trigger.initial_price > trigger.target_price {
            increase = false;
        }
        let coin_owned = vec![String::from(&coin.raw.id)];
        let coin_half_owned: Vec<_> = coin_owned.iter().map(String::as_str).collect();

        let currency_owned = vec![String::from(&currency.raw.name)];
        let currency_half_owned: Vec<_> = currency_owned.iter().map(String::as_str).collect();

        let price = client.price(&coin_half_owned, &currency_half_owned).await?;
        let price = price[&coin.raw.id][&currency.raw.name];
        if (increase && price >= trigger.target_price) || (!increase && price <= trigger.target_price){
            //println!("{} => {}\nOld Price: {}\nNew Price: {}\nChanged: {}", trigger.coin.to_uppercase(), trigger.currency.to_uppercase(), trigger.old_price, price, (price - trigger.old_price as f64).abs());
    
            Notification::new()
                .appname("JNA Monitor")
                .summary(&format!("{} => {}", coin.raw.id.to_uppercase(), currency.raw.name.to_uppercase()))
                .body(&format!("Initial Price: {}\nTarget Price: {}\nCurrent Price: {}\nDifference: {}", trigger.initial_price as i64, trigger.target_price as i64, price, (price - trigger.initial_price).abs() as i64))
                .icon("D:/Projects/Organisation/mywork/JNA_Monitor/icon.png")
                .show()?;
            
            client.delete_trigger(trigger.rowid).await?;
        } 
    }
    Ok(())
}
