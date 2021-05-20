use std::{rc::Rc, sync::{Arc, RwLock}};
use coingecko_requests::data::{Coin, VsCurrency};
use iced::{Button, Clipboard, Column, Command, HorizontalAlignment, Length, PickList, Row, Scrollable, Text, TextInput, button, pick_list, scrollable, text_input};

pub struct Flags {
    pub coins: Rc<Vec<coingecko_requests::data::Coin>>,
    pub currencies: Rc<Vec<coingecko_requests::data::VsCurrency>>,
    pub settings: Arc<RwLock<crate::settings::Settings>>,
}

#[derive(Debug, Clone)]
pub enum Message {
    TriggersUpdated(Vec<coingecko_requests::data::Trigger>),
    SaveTriggerClicked,
    CoinPicked(coingecko_requests::data::Coin),
    CurrencyPicked(coingecko_requests::data::VsCurrency),
    PriceInputChanged(String),
    TriggersAdded,
    DeleteTriggerClicked(i64),
    TriggerDeleted
}

#[derive(Debug, Clone)]
pub struct Gui {
    coins: Rc<Vec<coingecko_requests::data::Coin>>,
    currencies: Rc<Vec<coingecko_requests::data::VsCurrency>>,
    settings: Arc<RwLock<crate::settings::Settings>>,
    triggers: Vec<coingecko_requests::data::Trigger>,
    picked_coin: coingecko_requests::data::Coin,
    coin_picklist_state: pick_list::State<coingecko_requests::data::Coin>,
    picked_currency: coingecko_requests::data::VsCurrency,
    currency_picklist_state: pick_list::State<coingecko_requests::data::VsCurrency>,
    price_input_state: text_input::State,
    price_value: String,
    save_trigger_state: button::State,
    scrollable_state: scrollable::State,
    delete_button_states: Vec<button::State>,
}

impl Gui {
    pub fn new(flags: Flags) -> (Self, Command<Message>) {
        let picked_coin = flags.coins.iter().find(|coin| coin.raw.id == "bitcoin").cloned().unwrap();
        let picked_currency = flags.currencies.iter().find(|currency| currency.raw.name == "usd").cloned().unwrap();

        (Self{
            coins: flags.coins,
            currencies: flags.currencies,
            settings: flags.settings,
            triggers: Vec::new(),
            coin_picklist_state: Default::default(),
            picked_coin: picked_coin.clone(),
            picked_currency: picked_currency.clone(),
            currency_picklist_state: Default::default(),
            price_input_state: Default::default(),
            price_value: Default::default(),
            save_trigger_state: Default::default(),
            scrollable_state: Default::default(),
            delete_button_states: Vec::new(),
        }, Command::perform(update_triggers(), |result| Message::TriggersUpdated(result.unwrap())))
    }

    pub fn update(&mut self, message: Message, _clipboard: &mut Clipboard) -> Command<Message> {
        match message {
            Message::TriggersUpdated(vec) => {
                println!("Triggers updated. len = {}", vec.len());
                self.triggers = vec;
            }
            Message::PriceInputChanged(value) => {
                self.price_value = value;
            }
            Message::CoinPicked(picked) => {
                self.picked_coin = picked;
            }
            Message::CurrencyPicked(picked) => {
                self.picked_currency = picked;
            }
            Message::SaveTriggerClicked => {
                if let Ok(value) = self.price_value.parse::<f64>() {
                    return Command::perform(add_trigger(self.picked_coin.clone(), self.picked_currency.clone(), value), |x| { x.unwrap(); Message::TriggersAdded });
                }
            }
            Message::TriggersAdded => {
                return Command::perform(update_triggers(), |result| Message::TriggersUpdated(result.unwrap()));
            }
            Message::DeleteTriggerClicked(id) => {
                return Command::perform(delete_trigger(id), |result| { result.unwrap(); Message::TriggerDeleted });
            }
            Message::TriggerDeleted => {
                return Command::perform(update_triggers(), |result| Message::TriggersUpdated(result.unwrap()));
            }
        }
        Command::none()
    }

    pub fn view(&mut self) -> iced::Element<'_, Message> {
        let lock = self.settings.read().unwrap();
        let show_all_coins = lock.show_all_coins;
        let show_all_currencies = lock.show_all_currencies;
        let coins = if show_all_coins { self.coins.as_ref().clone() } else { self.coins.iter().filter(|coin| coin.favourite).cloned().collect() };
        let currencies = if show_all_currencies { self.currencies.as_ref().clone() } else { self.currencies.iter().filter(|coin| coin.favourite).cloned().collect() };

        self.delete_button_states = vec![Default::default(); self.triggers.len()];
        let mut delete_button_states = self.delete_button_states.iter_mut().collect::<Vec<_>>();

        let mut main_column = Column::new()
            .spacing(5)
            .width(Length::Fill)
            .height(Length::Fill);
        
        let mut trigger_settings_row = Row::new()
            .spacing(5)
            .width(Length::Shrink)
            .height(Length::Shrink);

        let mut coin_column = Column::new()
            .spacing(5)
            .width(Length::FillPortion(1)); 
        coin_column = coin_column.push(Text::new("Coin"));
        let coin_picklist = PickList::new(&mut self.coin_picklist_state, coins, Some(self.picked_coin.clone()), Message::CoinPicked).width(Length::Fill);
        coin_column = coin_column.push(coin_picklist);

        let mut vs_currency_column = Column::new()
            .spacing(5)
            .width(Length::FillPortion(1));
        vs_currency_column = vs_currency_column.push(Text::new("Currency"));
        let vs_currency_picklist = PickList::new(&mut self.currency_picklist_state, currencies, Some(self.picked_currency.clone()), Message::CurrencyPicked).width(Length::Fill);
        vs_currency_column = vs_currency_column.push(vs_currency_picklist);

        let mut price_input_column = Column::new()
            .spacing(5)
            .width(Length::FillPortion(1));
        price_input_column = price_input_column.push(Text::new("Enter a value"));
        let text_input_price = TextInput::new(&mut self.price_input_state,"200",&mut self.price_value ,Message::PriceInputChanged).width(Length::Fill).padding(5);
        price_input_column = price_input_column.push(text_input_price);

        trigger_settings_row = trigger_settings_row.push(coin_column);
        trigger_settings_row = trigger_settings_row.push(vs_currency_column);
        trigger_settings_row = trigger_settings_row.push(price_input_column);
        trigger_settings_row = trigger_settings_row.push(Button::new(&mut self.save_trigger_state, Text::new("Save").horizontal_alignment(HorizontalAlignment::Center)).on_press(Message::SaveTriggerClicked).width(Length::Fill).padding(17));

        main_column = main_column.push(trigger_settings_row);

        let mut scrollable = Scrollable::new(&mut self.scrollable_state)
            .width(Length::Fill)
            .height(Length::Fill);

        for trigger in self.triggers.iter() {
            let coin = self.coins.iter().find(|coin| coin.rowid == trigger.coin_id).cloned().unwrap();
            let currency = self.currencies.iter().find(|currency| currency.rowid == trigger.currency_id).cloned().unwrap();
            let initial_price = trigger.initial_price;
            let target_price = trigger.target_price;
            let mut trigger_row = Row::new().padding(5).spacing(5).width(Length::Fill);
            trigger_row = trigger_row.push(Button::new(delete_button_states.pop().unwrap(), Text::new("delete")).on_press(Message::DeleteTriggerClicked(trigger.rowid)));
            trigger_row = trigger_row.push(Text::new(format!("Trigger #{}: coin: {}, currency: {} from {} to {}", trigger.rowid, coin.raw.id, currency.raw.name, initial_price, target_price)));
            scrollable = scrollable.push(trigger_row);
        }

        main_column = main_column.push(scrollable);

        main_column.into()
    }
}

pub async fn update_triggers() -> Result<Vec<coingecko_requests::data::Trigger>, Box<dyn std::error::Error>> {
    let api_client = coingecko_requests::api_client::Client::new();
    let client = coingecko_requests::caching_client::Client::new(api_client).await?;

    Ok(client.get_all_triggers().await?)
}

pub async fn add_trigger(coin: Coin, currency: VsCurrency, value: f64) -> Result<(), Box<dyn std::error::Error>> {
    let api_client = coingecko_requests::api_client::Client::new();
    let client = coingecko_requests::caching_client::Client::new(api_client).await?;

    let data = client.price(&[coin.raw.id.as_str()], &[currency.raw.name.as_str()]).await?;

    client.add_trigger(coin.rowid, currency.rowid, data[&coin.raw.id][&currency.raw.name], value).await?;

    Ok(())
}

pub async fn delete_trigger(trigger_id: i64) -> Result<(), Box<dyn std::error::Error>> {
    let api_client = coingecko_requests::api_client::Client::new();
    let client = coingecko_requests::caching_client::Client::new(api_client).await?;

    client.delete_trigger(trigger_id).await?;

    Ok(())

}