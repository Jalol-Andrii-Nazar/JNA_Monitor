use std::{collections::HashMap, fmt::Display};

use chrono::{Local, NaiveDateTime};
use hotplot::chart::line::{self, data::{PlotSettings, Settings}};
use iced::{Canvas, Clipboard, Column, Command, Container, Element, Length, PickList, Row, Text, pick_list};
use line::data::DistanceValue;

pub struct GuiFlags {
    pub coins: Vec<coingecko_requests::data::RawCoin>,
    pub vs_currencies: Vec<coingecko_requests::data::RawVsCurrency>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawCoinWrapper(coingecko_requests::data::RawCoin);
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawVsCurrencyWrapper(coingecko_requests::data::RawVsCurrency);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimePeriod {
    Daily,
    Weekly,
    Monthly,
    Yearly,
    All
}

impl Default for TimePeriod {
    fn default() -> Self {
        Self::Weekly
    }
}

impl Display for TimePeriod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimePeriod::Daily => { write!(f, "Daily") }
            TimePeriod::Weekly => { write!(f, "Weekly") }
            TimePeriod::Monthly => { write!(f, "Monthly") }
            TimePeriod::Yearly => { write!(f, "Yearly") }
            TimePeriod::All => { write!(f, "All") }
        }
    }
}

impl TimePeriod {
    pub fn all() -> Vec<Self> {
        vec![Self::Daily, Self::Weekly, Self::Monthly, Self::Yearly, Self::All]
    }

    pub fn get_from_to(&self, current: u64) -> (u64, u64) {
        return match self {
            TimePeriod::Daily => {
                let to = current - (current % 60);
                let from = to - 60*60*24;
                (from, to)
            }
            TimePeriod::Weekly => {
                let to = current - (current % (60*60));
                let from = to - 60*60*24*7;
                (from, to)
            }
            TimePeriod::Monthly => {
                let to = current - (current % (60*60*24));
                let from = to - 60*60*24*30;
                (from, to)
            }
            TimePeriod::Yearly => {
                let to = current - (current % (60*60*24));
                let from = to - 60*60*24*365;
                (from, to)
            }
            TimePeriod::All => {
                let to = current - (current % (60*60*24));
                let from = 0;
                (from, to)
            }
        }
    }
}

pub struct Gui {
    coins: Vec<RawCoinWrapper>,
    vs_currencies: Vec<RawVsCurrencyWrapper>,
    time_periods: Vec<TimePeriod>,
    latest_data_request_timestamp: u64,
    data: Result<Option<Vec<(NaiveDateTime, f64)>>, Box<dyn std::error::Error>>,
    picked_coin: RawCoinWrapper,
    picked_vs_currency: RawVsCurrencyWrapper,
    coin_picklist_state: pick_list::State<RawCoinWrapper>,
    vs_currency_picklist_state: pick_list::State<RawVsCurrencyWrapper>,
    time_period_packlist_state: pick_list::State<TimePeriod>,
    time_period: TimePeriod
}

#[derive(Debug, Clone)]
pub enum GuiMessage {
    CoinPicked(RawCoinWrapper),
    VsCurrencyPicked(RawVsCurrencyWrapper),
    TimePeriodPicked(TimePeriod),
    DataLoaded(Vec<(NaiveDateTime, f64)>, u64),
    DataLoadFailed(String, u64),
    ChartMessage(line::data::Message)
}

impl Gui {
    pub fn new(flags: GuiFlags) -> (Self, Command<GuiMessage>) {
        let picked_coin: RawCoinWrapper = RawCoinWrapper(flags.coins.iter().find(|coin| coin.id == "bitcoin").cloned().unwrap());
        let picked_vs_currency: RawVsCurrencyWrapper = RawVsCurrencyWrapper(flags.vs_currencies.iter().find(|currency| currency.name == "usd").cloned().unwrap());
        let time_period: TimePeriod = Default::default();
        let timestamp = Local::now().timestamp() as u64;
        let (from, to) = time_period.get_from_to(timestamp as u64);
        println!("From {} to {}", from, to);
        (Self {
            coins: flags.coins.iter().map(|coin| RawCoinWrapper(coin.clone())).collect(),
            vs_currencies: flags.vs_currencies.iter().map(|currency| RawVsCurrencyWrapper(currency.clone())).collect(),
            time_periods: TimePeriod::all(),
            latest_data_request_timestamp: timestamp,
            data: Ok(None),
            picked_coin: picked_coin.clone(),
            picked_vs_currency: picked_vs_currency.clone(),
            coin_picklist_state: Default::default(),
            vs_currency_picklist_state: Default::default(),
            time_period_packlist_state: Default::default(),
            time_period: time_period.clone()
        }, Command::perform(load_data(picked_coin.0.id.clone(), picked_vs_currency.0.name.clone(), from, to, timestamp), |x| x))
    }

    pub fn update(&mut self, message: GuiMessage, _clipboard: &mut Clipboard) -> Command<GuiMessage> {
        match message {
            GuiMessage::CoinPicked(picked) => {
                let timestamp = Local::now().timestamp() as u64;
                self.latest_data_request_timestamp = timestamp;
                self.picked_coin = picked;
                self.data = Ok(None);
                let (from, to) = self.time_period.get_from_to(Local::now().timestamp() as u64);
                Command::perform(load_data(self.picked_coin.0.id.clone(), self.picked_vs_currency.0.name.clone(), from, to, timestamp), |x| x)
            }
            GuiMessage::VsCurrencyPicked(picked) => {
                let timestamp = Local::now().timestamp() as u64;
                self.latest_data_request_timestamp = timestamp;
                self.picked_vs_currency = picked;
                self.data = Ok(None);
                let (from, to) = self.time_period.get_from_to(Local::now().timestamp() as u64);
                Command::perform(load_data(self.picked_coin.0.id.clone(), self.picked_vs_currency.0.name.clone(), from, to, timestamp), |x| x)
            }
            GuiMessage::TimePeriodPicked(picked) => {
                let timestamp = Local::now().timestamp() as u64;
                self.latest_data_request_timestamp = timestamp;
                self.time_period = picked;
                self.data = Ok(None);
                let (from, to) = self.time_period.get_from_to(Local::now().timestamp() as u64);
                Command::perform(load_data(self.picked_coin.0.id.clone(), self.picked_vs_currency.0.name.clone(), from, to, timestamp), |x| x)
            }
            GuiMessage::DataLoaded(data, timestamp) => {
                if self.latest_data_request_timestamp == timestamp {
                    self.data = Ok(Some(data));
                }
                Command::none()
            }
            GuiMessage::DataLoadFailed(err, timestamp) => {
                if self.latest_data_request_timestamp == timestamp {
                    self.data = Err(err.into());
                }
                Command::none()
            }
            GuiMessage::ChartMessage(_) => {
                Command::none()
            }
        }
    }

    pub fn view(&mut self) -> iced::Element<'_, GuiMessage> {
        let mut main_column = Column::new().spacing(5);

        let mut chart_settings_row = Row::new().spacing(5).width(Length::Shrink);

        let mut coin_column = Column::new().spacing(5).width(Length::FillPortion(1));
        coin_column = coin_column.push(Text::new("Coin id"));
        let coin_picklist = PickList::new(&mut self.coin_picklist_state, &self.coins, Some(self.picked_coin.clone()), GuiMessage::CoinPicked).width(Length::Fill);
        coin_column = coin_column.push(coin_picklist);

        let mut vs_currency_column = Column::new().spacing(5).width(Length::FillPortion(1));
        vs_currency_column = vs_currency_column.push(Text::new("Currency name"));
        let vs_currency_picklist = PickList::new(&mut self.vs_currency_picklist_state, &self.vs_currencies, Some(self.picked_vs_currency.clone()), GuiMessage::VsCurrencyPicked).width(Length::Fill);
        vs_currency_column = vs_currency_column.push(vs_currency_picklist);

        let mut time_period_column = Column::new().spacing(5).width(Length::FillPortion(1));
        time_period_column = time_period_column.push(Text::new("Time period"));
        let time_period_picklist = PickList::new(&mut self.time_period_packlist_state, &self.time_periods, Some(self.time_period.clone()), GuiMessage::TimePeriodPicked).width(Length::Fill);
        time_period_column = time_period_column.push(time_period_picklist);
        
        chart_settings_row = chart_settings_row.push(coin_column);
        chart_settings_row = chart_settings_row.push(vs_currency_column);
        chart_settings_row = chart_settings_row.push(time_period_column);

        main_column = main_column.push(chart_settings_row);

        match self.data {
            Ok(Some(ref data)) => {
                if data.is_empty() {
                    main_column = main_column.push(Text::new("There is no data for this period of time!"));
                } else {
                    let settings = Settings {
                        title: format!("{} to {} graph", self.picked_coin.0.id, self.picked_vs_currency.0.name),
                        min_x_label_distance: DistanceValue::Fixed(200.0),
                        min_y_label_distance: DistanceValue::Fixed(100.0),
                        ..Default::default()
                    };
                    let min_x_value = data.iter().map(|(d, _)| *d).min().unwrap();
                    let max_x_value = data.iter().map(|(d, _)| *d).max().unwrap();
                    let min_y_value = data.iter().map(|(_, p)| *p).min_by(|f1, f2| f1.total_cmp(f2)).unwrap().floor();
                    let max_y_value = data.iter().map(|(_, p)| *p).max_by(|f1, f2| f1.total_cmp(f2)).unwrap().ceil();
                    let plot_settings = PlotSettings {
                        point_size1: 4.0,
                        point_size2: 5.5,
                        point_size3: 7.0,
                        ..Default::default()
                    };
                    let mut plot_data = HashMap::new();
                    plot_data.insert(plot_settings, data.clone());
                    let chart = line::Chart::new(
                        settings,
                        min_x_value,
                        max_x_value,
                        min_y_value,
                        max_y_value,
                        plot_data
                    );
            
                    let canvas = Canvas::new(chart).width(Length::Fill).height(Length::Fill);
                    let container: Container<_> = Container::new(canvas)
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .center_x()
                        .center_y();
                    let container_elem: Element<_> = container.into();
            
                    main_column = main_column.push(container_elem.map(GuiMessage::ChartMessage));
                }
            }
            Ok(None) => {
                main_column = main_column.push(Text::new("Loading data, please wait..."));
            }
            Err(ref err) => {
                main_column = main_column.push(Text::new("Failed to load data! See the erorr below..."));
                main_column = main_column.push(Text::new(err.to_string()));
            }
        }

        main_column.into()
    }
}

async fn load_data(id: String, vs_currency: String, from: u64, to: u64, timestamp: u64) -> GuiMessage {
    let client = coingecko_requests::api_client::Client::new();
    let result = client.market_chart(&id, &vs_currency, from, to)
        .await
        .map(|coin_range| coin_range.prices
            .into_iter()
            .map(|(timestamp, price)| (NaiveDateTime::from_timestamp(timestamp as i64 / 1000, 0), price))
            .collect::<Vec<_>>());
    match result {
        Ok(data) => {
            GuiMessage::DataLoaded(data, timestamp)
        }
        Err(err) => {
            GuiMessage::DataLoadFailed(err.to_string(), timestamp)
        }
    }
}

impl Display for RawCoinWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.id)
    }
}

impl Display for RawVsCurrencyWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.name)
    }
}