use std::{rc::Rc, sync::{Arc, RwLock}};
use iced::{Canvas, Clipboard, Column, Command, Length, PickList, Row, Text, pick_list, TextInput, text_input, Button, button, HorizontalAlignment};

pub struct Flags {
    pub coins: Rc<Vec<coingecko_requests::data::Coin>>,
    pub currencies: Rc<Vec<coingecko_requests::data::VsCurrency>>,
    pub settings: Arc<RwLock<crate::settings::Settings>>,
}

#[derive(Debug, Clone)]
pub enum Message {
    SaveTriggerClicked,
    CoinPicked(coingecko_requests::data::Coin),
    CurrencyPicked(coingecko_requests::data::VsCurrency),
    PriceFromInputChanged(String),
    PriceToInputChanged(String),
}

#[derive(Debug, Clone)]
pub struct Gui {
    coins: Rc<Vec<coingecko_requests::data::Coin>>,
    currencies: Rc<Vec<coingecko_requests::data::VsCurrency>>,
    settings: Arc<RwLock<crate::settings::Settings>>,
    picked_coin: coingecko_requests::data::Coin,
    coin_picklist_state: pick_list::State<coingecko_requests::data::Coin>,
    picked_currency: coingecko_requests::data::VsCurrency,
    currency_picklist_state: pick_list::State<coingecko_requests::data::VsCurrency>,
    price_from_input_state: text_input::State,
    price_from_value: String,
    price_to_input_state: text_input::State,
    price_to_value: String,
    save_trigger_state: button::State,
}

impl Gui {
    pub fn new(flags: Flags) -> (Self, Command<Message>) {
        let picked_coin = flags.coins.iter().find(|coin| coin.raw.id == "bitcoin").cloned().unwrap();
        let picked_currency = flags.currencies.iter().find(|currency| currency.raw.name == "usd").cloned().unwrap();

        (Self{
            coins: flags.coins,
            currencies: flags.currencies,
            settings: flags.settings,
            coin_picklist_state: Default::default(),
            picked_coin: picked_coin.clone(),
            picked_currency: picked_currency.clone(),
            currency_picklist_state: Default::default(),
            price_from_input_state: Default::default(),
            price_from_value: Default::default(),
            price_to_input_state: Default::default(),
            price_to_value: Default::default(),
            save_trigger_state: Default::default(),
        }, Command::none())
    }

    pub fn update(&mut self, message: Message, _clipboard: &mut Clipboard) -> Command<Message> {
        match message {
            Message::PriceFromInputChanged(value) => {
                self.price_from_value = value;
            }
            Message::PriceToInputChanged(value) => {
                self.price_to_value = value;
            }
            Message::CoinPicked(picked) => {
                self.picked_coin = picked;
            }
            Message::CurrencyPicked(picked) => {
                self.picked_currency = picked;
            }
            Message::SaveTriggerClicked => {}
        }
        Command::none()
    }

    pub fn view(&mut self) -> iced::Element<'_, Message> {
        let lock = self.settings.read().unwrap();
        let show_all_coins = lock.show_all_coins;
        let show_all_currencies = lock.show_all_currencies;
        let coins = if show_all_coins { self.coins.as_ref().clone() } else { self.coins.iter().filter(|coin| coin.favourite).cloned().collect() };
        let currencies = if show_all_currencies { self.currencies.as_ref().clone() } else { self.currencies.iter().filter(|coin| coin.favourite).cloned().collect() };

        //Text::new("Hello Triggers").into();
        let mut main_column = Row::new().spacing(5);
        let mut trigger_settings_row = Row::new().spacing(5).width(Length::Shrink);


        let mut coin_column = Column::new().spacing(5).width(Length::FillPortion(1)); 
        coin_column = coin_column.push(Text::new("Coin"));
        let coin_picklist = PickList::new(&mut self.coin_picklist_state, coins, Some(self.picked_coin.clone()), Message::CoinPicked).width(Length::Fill);
        coin_column = coin_column.push(coin_picklist);

        let mut vs_currency_column = Column::new().spacing(5).width(Length::FillPortion(1));
        vs_currency_column = vs_currency_column.push(Text::new("Currency"));
        let vs_currency_picklist = PickList::new(&mut self.currency_picklist_state, currencies, Some(self.picked_currency.clone()), Message::CurrencyPicked).width(Length::Fill);
        vs_currency_column = vs_currency_column.push(vs_currency_picklist);

        let mut price__from_input_column = Column::new().spacing(5).width(Length::FillPortion(1));
        price__from_input_column = price__from_input_column.push(Text::new("From"));
        let text_input_price = TextInput::new(&mut self.price_from_input_state,"200",&mut self.price_from_value ,Message::PriceFromInputChanged).width(Length::Fill).padding(5);
        price__from_input_column = price__from_input_column.push(text_input_price);

        let mut price_to_input_column = Column::new().spacing(5).width(Length::FillPortion(1));
        price_to_input_column = price_to_input_column.push(Text::new("To"));
        let text_input_price = TextInput::new(&mut self.price_to_input_state,"1000",&mut self.price_to_value ,Message::PriceToInputChanged).width(Length::Fill).padding(5);
        price_to_input_column = price_to_input_column.push(text_input_price);


        trigger_settings_row = trigger_settings_row.push(coin_column);
        trigger_settings_row = trigger_settings_row.push(vs_currency_column);
        trigger_settings_row = trigger_settings_row.push(price__from_input_column);
        trigger_settings_row = trigger_settings_row.push(price_to_input_column);
        trigger_settings_row = trigger_settings_row.push(  Button::new(&mut self.save_trigger_state, Text::new("Save").horizontal_alignment(HorizontalAlignment::Center)).on_press(Message::SaveTriggerClicked).width(Length::Fill).padding(17));

        main_column = main_column.push(trigger_settings_row);

        main_column.into()
    }
}