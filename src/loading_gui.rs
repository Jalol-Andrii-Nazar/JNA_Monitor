use std::time::Duration;

use chrono::{NaiveDate, NaiveDateTime};
use iced::{Application, Clipboard, Column, Command, Text, executor::Default};

enum LoadingState {
    Initilizing,
    Errored,
    Successful(crate::gui::Gui)
}

#[derive(Debug)]
pub enum GuiMessage {
    IdsLoaded(Vec<String>),
    VsCurrenciesLoaded(Vec<String>),
    BtcToUsdLoaded(Vec<(NaiveDate, f64)>),
    Error(String),
    GuiMessage(crate::gui::GuiMessage)
}

pub struct Gui {
    messages: Vec<String>,
    state: LoadingState,
    ids: Option<Vec<String>>,
    vs_currencies: Option<Vec<String>>,
    btc_to_usd: Option<Vec<(NaiveDate, f64)>>
}

impl Application for Gui {
    type Executor = Default;

    type Message = GuiMessage;

    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let messages = vec![format!("Loading '{}' v. {}. Please wait...", crate::NAME, crate::VERSION)];
        (Self {
            messages,
            state: LoadingState::Initilizing,
            ids: None,
            vs_currencies: None,
            btc_to_usd: None
        }, Command::perform(load_ids(), unwrap_result))
    }

    fn title(&self) -> String {
        format!("Loading...")
    }

    fn update(&mut self, message: Self::Message, clipboard: &mut Clipboard) -> Command<Self::Message> {
        match message {
            GuiMessage::IdsLoaded(ids) => {
                self.ids = Some(ids);
                self.messages.push(format!("Ids loaded successfully..."));
                Command::perform(load_vs_currencies(), unwrap_result)
            }
            GuiMessage::VsCurrenciesLoaded(vs_currencies) => {
                self.vs_currencies = Some(vs_currencies);
                self.messages.push(format!("VsCurrencies loaded successfully..."));
                Command::perform(load_btc_to_usd(), unwrap_result)
            }
            GuiMessage::BtcToUsdLoaded(btc_to_usd) => {
                self.btc_to_usd = Some(btc_to_usd);
                self.messages.push(format!("BtcToUsd loaded successfully..."));
                self.messages.push(format!("Starting the GUI..."));
                let (gui, gui_message) = crate::gui::Gui::new(crate::gui::GuiFlags {
                    ids: self.ids.take().unwrap(),
                    vs_currencies: self.vs_currencies.take().unwrap(),
                    btc_to_usd: self.btc_to_usd.take().unwrap()
                });
                self.state = LoadingState::Successful(gui);
                gui_message.map(Self::Message::GuiMessage)
            }
            GuiMessage::Error(error) => {
                self.state = LoadingState::Errored;
                self.messages.push(format!("An error happened! {}", error));
                Command::none()
            }
            GuiMessage::GuiMessage(msg) => {
                if let LoadingState::Successful(ref mut gui) = self.state {
                    gui.update(msg, clipboard).map(Self::Message::GuiMessage)
                } else {
                    panic!("Should not happen")
                }
            }
        }
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        if let LoadingState::Successful(ref mut gui) = self.state {
            gui.view().map(GuiMessage::GuiMessage)
        } else {
            let mut column = Column::new();
            for message in self.messages.iter() {
                column = column.push(Text::new(message));
            }
            column.into()
        }
    }
}

fn unwrap_result(result: Result<GuiMessage, Box<dyn std::error::Error>>) -> GuiMessage {
    match result {
        Ok(message) => { message }
        Err(err) => { GuiMessage::Error(err.to_string()) }
    }
}

async fn load_ids() -> Result<GuiMessage, Box<dyn std::error::Error>> {
    let coins = coingecko_requests::client::Client::new().coins_list().await?;
    let ids = coins.into_iter().map(|coin| coin.id).collect::<Vec<_>>();
    tokio::time::sleep(Duration::from_secs(1)).await;
    Ok(GuiMessage::IdsLoaded(ids))
}

async fn load_vs_currencies() -> Result<GuiMessage, Box<dyn std::error::Error>> {
    let vs_currencies = coingecko_requests::client::Client::new().simple_supported_vs_currencies().await?;
    tokio::time::sleep(Duration::from_secs(1)).await;
    Ok(GuiMessage::VsCurrenciesLoaded(vs_currencies))
}

async fn load_btc_to_usd() -> Result<GuiMessage, Box<dyn std::error::Error>> {
    let btc_to_usd = coingecko_requests::client::Client::new().coins_id_market_chart_range("bitcoin", "usd", 1392577232, 1422577232)
        .await?
        .prices
        .into_iter()
        .map(|(timestamp, price)| (NaiveDateTime::from_timestamp(timestamp as i64 / 1000, 0).date(), price))
        .collect::<Vec<_>>();
    tokio::time::sleep(Duration::from_secs(1)).await;
    Ok(GuiMessage::BtcToUsdLoaded(btc_to_usd))
}