use iced::{Application, Clipboard, Column, Command, Text, executor::Default};

enum GuiState {
    Initilizing,
    Errored,
    Initialized(crate::gui::Gui)
}

#[derive(Debug)]
pub enum GuiMessage {
    CoinsLoaded(Vec<coingecko_requests::data::RawCoin>),
    VsCurrenciesLoaded(Vec<coingecko_requests::data::RawVsCurrency>),
    Error(String),
    GuiMessage(crate::gui::GuiMessage)
}

pub struct Gui {
    messages: Vec<String>,
    state: GuiState,
    coins: Option<Vec<coingecko_requests::data::RawCoin>>,
    vs_currencies: Option<Vec<coingecko_requests::data::RawVsCurrency>>,
}

impl Application for Gui {
    type Executor = Default;

    type Message = GuiMessage;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let messages = vec![format!("Loading '{}' v. {}. Please wait...", crate::NAME, crate::VERSION), format!("Loading ids...")];
        (Self {
            messages,
            state: GuiState::Initilizing,
            coins: None,
            vs_currencies: None,
        }, Command::perform(load_coins(), unwrap_result))
    }

    fn title(&self) -> String {
        if let GuiState::Initialized(ref _gui) = self.state {
            format!("{} v.{}", crate::NAME, crate::VERSION)
        } else {
            format!("Loading...")
        }
    }

    fn update(&mut self, message: Self::Message, clipboard: &mut Clipboard) -> Command<Self::Message> {
        match message {
            GuiMessage::CoinsLoaded(coins) => {
                self.coins = Some(coins);
                self.messages.push(format!("Coins loaded successfully..."));
                self.messages.push(format!("Loading currencies..."));
                Command::perform(load_vs_currencies(), unwrap_result)
            }
            GuiMessage::VsCurrenciesLoaded(vs_currencies) => {
                self.vs_currencies = Some(vs_currencies);
                self.messages.push(format!("Currencies loaded successfully..."));
                self.messages.push(format!("Starting the GUI..."));
                let (gui, gui_message) = crate::gui::Gui::new(crate::gui::GuiFlags {
                    coins: self.coins.take().unwrap(),
                    vs_currencies: self.vs_currencies.take().unwrap(),
                    });
                self.state = GuiState::Initialized(gui);
                gui_message.map(Self::Message::GuiMessage)
            }
            GuiMessage::Error(error) => {
                self.state = GuiState::Errored;
                self.messages.push(format!("An error happened! {}", error));
                Command::none()
            }
            GuiMessage::GuiMessage(msg) => {
                if let GuiState::Initialized(ref mut gui) = self.state {
                    gui.update(msg, clipboard).map(Self::Message::GuiMessage)
                } else {
                    panic!("SHOULD NOT HAPPEN: gui message cannot be received before initialization is completed.")
                }
            }
        }
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        if let GuiState::Initialized(ref mut gui) = self.state {
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

async fn load_coins() -> Result<GuiMessage, Box<dyn std::error::Error>> {
    let api_client = coingecko_requests::api_client::Client::new();
    let mut caching_client = coingecko_requests::caching_client::Client::new(api_client).await?;
    let coins = caching_client.favourite_coins().await?.into_iter().map(|coin| coin.raw).collect();
    Ok(GuiMessage::CoinsLoaded(coins))
}

async fn load_vs_currencies() -> Result<GuiMessage, Box<dyn std::error::Error>> {
    let api_client = coingecko_requests::api_client::Client::new();
    let mut caching_client = coingecko_requests::caching_client::Client::new(api_client).await?;
    let vs_currencies = caching_client.favourite_vs_currencies().await?.into_iter().map(|currency| currency.raw).collect();
    Ok(GuiMessage::VsCurrenciesLoaded(vs_currencies))
}
