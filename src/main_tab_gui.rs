use std::{collections::HashMap, fmt::Display, rc::Rc, sync::{Arc, RwLock}};

use chrono::{Local, NaiveDate, NaiveDateTime};
use hotplot::chart::line::{self, data::{PlotSettings, PlotThemeSettings, Settings, ThemeSettings}};
use iced::{Canvas, Clipboard, Column, Command, Container, Element, Length, PickList, Row, Text, pick_list};
use line::data::DistanceValue;

pub struct Flags {
    pub coins: Rc<Vec<coingecko_requests::data::Coin>>,
    pub currencies: Rc<Vec<coingecko_requests::data::VsCurrency>>,
    pub settings: Arc<RwLock<crate::settings::Settings>>
}

// #[derive(Debug, Clone, PartialEq, Eq)]
// pub struct RawCoinWrapper(coingecko_requests::data::RawCoin);
// #[derive(Debug, Clone, PartialEq, Eq)]
// pub struct RawVsCurrencyWrapper(coingecko_requests::data::RawVsCurrency);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimePeriod {
    Daily,
    Weekly,
    Monthly,
    Yearly,
    All,
    Custom
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
            TimePeriod::Custom => { write!(f, "Custom") }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DateParts {
    year: u32,
    month: u32,
    day: u32
}

impl DateParts {
    fn as_timestamp(&self) -> Option<u64> {
        NaiveDate::from_ymd_opt(self.year as i32, self.month, self.day)
            .map(|date| date.and_hms(0, 0, 0)
            .timestamp() as u64)
    }

    fn with_year(&self, year: u32) -> Self {
        Self {
            year,
            ..*self
        }
    }

    fn with_month(&self, month: u32) -> Self {
        Self {
            month,
            ..*self
        }
    }

    fn with_day(&self, day: u32) -> Self {
        Self {
            day,
            ..*self
        }
    }
}

impl TimePeriod {
    pub fn all() -> Vec<Self> {
        vec![Self::Daily, Self::Weekly, Self::Monthly, Self::Yearly, Self::All, Self::Custom]
    }

    pub fn get_from_to(&self, current: u64, default_from: u64, default_to: u64) -> (u64, u64) {
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
            TimePeriod::Custom => {
                (default_from, default_to)
            }
        }
    }
}

pub struct Gui {
    coins: Rc<Vec<coingecko_requests::data::Coin>>,
    currencies: Rc<Vec<coingecko_requests::data::VsCurrency>>,
    settings: Arc<RwLock<crate::settings::Settings>>,
    time_periods: Vec<TimePeriod>,
    latest_data_request_timestamp: u64,
    data: Result<Option<Vec<(NaiveDateTime, f64)>>, Box<dyn std::error::Error>>,
    picked_coin: coingecko_requests::data::Coin,
    picked_currency: coingecko_requests::data::VsCurrency,
    coin_picklist_state: pick_list::State<coingecko_requests::data::Coin>,
    currency_picklist_state: pick_list::State<coingecko_requests::data::VsCurrency>,
    time_period_packlist_state: pick_list::State<TimePeriod>,
    date_from_year_picklist_state: pick_list::State<u32>,
    date_from_month_picklist_state: pick_list::State<u32>,
    date_from_day_picklist_state: pick_list::State<u32>,
    date_to_year_picklist_state: pick_list::State<u32>,
    date_to_month_picklist_state: pick_list::State<u32>,
    date_to_day_picklist_state: pick_list::State<u32>,
    years: Vec<u32>,
    months: Vec<u32>,
    days: Vec<u32>,
    time_period: TimePeriod,
    date_from: DateParts,
    date_to: DateParts
}

#[derive(Debug, Clone)]
pub enum Message {
    CoinPicked(coingecko_requests::data::Coin),
    CurrencyPicked(coingecko_requests::data::VsCurrency),
    TimePeriodPicked(TimePeriod),
    DataLoaded(Vec<(NaiveDateTime, f64)>, u64),
    DataLoadFailed(String, u64),
    ChartMessage(line::data::Message),
    DateFromYearUpdated(u32),
    DateFromMonthUpdated(u32),
    DateFromDayUpdated(u32),
    DateToYearUpdated(u32),
    DateToMonthUpdated(u32),
    DateToDayUpdated(u32),
}

impl Gui {
    pub fn new(flags: Flags) -> (Self, Command<Message>) {
        let picked_coin = flags.coins.iter().find(|coin| coin.raw.id == "bitcoin").cloned().unwrap();
        let picked_currency = flags.currencies.iter().find(|currency| currency.raw.name == "usd").cloned().unwrap();
        let date_from = DateParts {
            year: 2017,
            month: 1,
            day: 1
        };
        let date_to = DateParts {
            year: 2018,
            month: 1,
            day: 1
        };
        let time_period: TimePeriod = Default::default();
        let timestamp = Local::now().timestamp() as u64;
        let (from, to) = time_period.get_from_to(Local::now().timestamp() as u64, date_from.as_timestamp().unwrap(), date_to.as_timestamp().unwrap());
        println!("From {} to {}", from, to);
        (Self {
            coins: flags.coins,
            currencies: flags.currencies,
            settings: flags.settings,
            time_periods: TimePeriod::all(),
            latest_data_request_timestamp: timestamp,
            data: Ok(None),
            picked_coin: picked_coin.clone(),
            picked_currency: picked_currency.clone(),
            coin_picklist_state: Default::default(),
            currency_picklist_state: Default::default(),
            time_period_packlist_state: Default::default(),
            time_period: time_period.clone(),
            date_from_year_picklist_state: Default::default(),
            date_from_month_picklist_state: Default::default(),
            date_from_day_picklist_state: Default::default(),
            date_to_year_picklist_state: Default::default(),
            date_to_month_picklist_state: Default::default(),
            date_to_day_picklist_state: Default::default(),
            years: (2013..=2021).collect(),
            months: (1..=12).collect(),
            days: (1..=31).collect(),
            date_from,
            date_to
        }, Command::perform(load_data(picked_coin.raw.id.clone(), picked_currency.raw.name.clone(), from, to, timestamp), |x| x))
    }

    pub fn update(&mut self, message: Message, _clipboard: &mut Clipboard) -> Command<Message> {
        fn update_dates(gui: &mut Gui, new_from_date: DateParts, new_to_date: DateParts) -> Command<Message> {
            gui.date_from = new_from_date;
            gui.date_to = new_to_date;
            if let Some((timestamp_from, timestamp_to)) = new_from_date.as_timestamp().zip(new_to_date.as_timestamp()) {
                if timestamp_from < timestamp_to {
                    gui.data = Ok(None);
                    let timestamp = Local::now().timestamp() as u64;
                    gui.latest_data_request_timestamp = timestamp;    
                    return Command::perform(load_data(gui.picked_coin.raw.id.clone(), gui.picked_currency.raw.name.clone(), timestamp_from, timestamp_to, timestamp), |x| x);
                }
            }
            gui.data = Err(From::from("Invalid date(s)!"));
            Command::none()
        }
        match message {
            Message::CoinPicked(picked) => {
                let timestamp = Local::now().timestamp() as u64;
                self.latest_data_request_timestamp = timestamp;
                self.picked_coin = picked;
                self.data = Ok(None);
                let (from, to) = self.time_period.get_from_to(Local::now().timestamp() as u64, self.date_from.as_timestamp().unwrap(), self.date_to.as_timestamp().unwrap());
                Command::perform(load_data(self.picked_coin.raw.id.clone(), self.picked_currency.raw.name.clone(), from, to, timestamp), |x| x)
            }
            Message::CurrencyPicked(picked) => {
                let timestamp = Local::now().timestamp() as u64;
                self.latest_data_request_timestamp = timestamp;
                self.picked_currency = picked;
                self.data = Ok(None);
                let (from, to) = self.time_period.get_from_to(Local::now().timestamp() as u64, self.date_from.as_timestamp().unwrap(), self.date_to.as_timestamp().unwrap());
                Command::perform(load_data(self.picked_coin.raw.id.clone(), self.picked_currency.raw.name.clone(), from, to, timestamp), |x| x)
            }
            Message::TimePeriodPicked(picked) => {
                let timestamp = Local::now().timestamp() as u64;
                self.latest_data_request_timestamp = timestamp;
                self.time_period = picked;
                self.data = Ok(None);
                let (from, to) = self.time_period.get_from_to(Local::now().timestamp() as u64, self.date_from.as_timestamp().unwrap(), self.date_to.as_timestamp().unwrap());
                Command::perform(load_data(self.picked_coin.raw.id.clone(), self.picked_currency.raw.name.clone(), from, to, timestamp), |x| x)
            }
            Message::DataLoaded(data, timestamp) => {
                if self.latest_data_request_timestamp == timestamp {
                    self.data = Ok(Some(data));
                }
                Command::none()
            }
            Message::DataLoadFailed(err, timestamp) => {
                if self.latest_data_request_timestamp == timestamp {
                    self.data = Err(err.into());
                }
                Command::none()
            }
            Message::DateFromYearUpdated(new_year) => {
                update_dates(self, self.date_from.with_year(new_year), self.date_to)
            }
            Message::DateFromMonthUpdated(new_month) => {
                update_dates(self, self.date_from.with_month(new_month), self.date_to)
            }
            Message::DateFromDayUpdated(new_day) => {
                update_dates(self, self.date_from.with_day(new_day), self.date_to)
            }
            Message::DateToYearUpdated(new_year) => {
                update_dates(self, self.date_from, self.date_to.with_year(new_year))
            }
            Message::DateToMonthUpdated(new_month) => {
                update_dates(self, self.date_from, self.date_to.with_month(new_month))
            }
            Message::DateToDayUpdated(new_day) => {
                update_dates(self, self.date_from, self.date_to.with_day(new_day))
            }
            Message::ChartMessage(_) => {
                Command::none()
            }
        }
    }

    pub fn view(&mut self) -> iced::Element<'_, Message> {
        let lock = self.settings.read().unwrap();
        let theme = lock.theme;
        let show_all_coins = lock.show_all_coins;
        let show_all_currencies = lock.show_all_currencies;
        let coins = if show_all_coins { self.coins.as_ref().clone() } else { self.coins.iter().filter(|coin| coin.favourite).cloned().collect() };
        let currencies = if show_all_currencies { self.currencies.as_ref().clone() } else { self.currencies.iter().filter(|coin| coin.favourite).cloned().collect() };

        let mut main_column = Column::new().spacing(5);

        let mut chart_settings_row = Row::new().spacing(5).width(Length::Shrink);

        let mut coin_column = Column::new().spacing(5).width(Length::FillPortion(1));
        coin_column = coin_column.push(Text::new("Coin"));
        let coin_picklist = PickList::new(&mut self.coin_picklist_state, coins, Some(self.picked_coin.clone()), Message::CoinPicked).width(Length::Fill).style(theme);
        coin_column = coin_column.push(coin_picklist);

        let mut vs_currency_column = Column::new().spacing(5).width(Length::FillPortion(1));
        vs_currency_column = vs_currency_column.push(Text::new("Currency"));
        let vs_currency_picklist = PickList::new(&mut self.currency_picklist_state, currencies, Some(self.picked_currency.clone()), Message::CurrencyPicked).width(Length::Fill).style(theme);
        vs_currency_column = vs_currency_column.push(vs_currency_picklist);

        let mut time_period_column = Column::new().spacing(5).width(Length::FillPortion(1));
        time_period_column = time_period_column.push(Text::new("Time period"));
        let time_period_picklist = PickList::new(&mut self.time_period_packlist_state, &self.time_periods, Some(self.time_period.clone()), Message::TimePeriodPicked).width(Length::Fill).style(theme);
        time_period_column = time_period_column.push(time_period_picklist);
        
        chart_settings_row = chart_settings_row.push(coin_column);
        chart_settings_row = chart_settings_row.push(vs_currency_column);
        chart_settings_row = chart_settings_row.push(time_period_column);

        main_column = main_column.push(chart_settings_row);

        if let TimePeriod::Custom = self.time_period {
            let mut dates_row = Row::new().spacing(5).width(Length::Shrink);

            let mut from_year_column = Column::new().spacing(5).width(Length::FillPortion(1));
            from_year_column = from_year_column.push(Text::new("Year"));
            let from_year_picklist = PickList::new(&mut self.date_from_year_picklist_state, &self.years, Some(self.date_from.year), Message::DateFromYearUpdated).width(Length::Fill).style(theme);
            from_year_column = from_year_column.push(from_year_picklist);

            let mut from_month_column = Column::new().spacing(5).width(Length::FillPortion(1));
            from_month_column = from_month_column.push(Text::new("Month"));
            let from_month_picklist = PickList::new(&mut self.date_from_month_picklist_state, &self.months, Some(self.date_from.month), Message::DateFromMonthUpdated).width(Length::Fill).style(theme);
            from_month_column = from_month_column.push(from_month_picklist);
            
            let mut from_day_column = Column::new().spacing(5).width(Length::FillPortion(1));
            from_day_column = from_day_column.push(Text::new("Day"));
            let from_day_picklist = PickList::new(&mut self.date_from_day_picklist_state, &self.days, Some(self.date_from.day), Message::DateFromDayUpdated).width(Length::Fill).style(theme);
            from_day_column = from_day_column.push(from_day_picklist);

            let mut to_year_column = Column::new().spacing(5).width(Length::FillPortion(1));
            to_year_column = to_year_column.push(Text::new("Year"));
            let to_year_picklist = PickList::new(&mut self.date_to_year_picklist_state, &self.years, Some(self.date_to.year), Message::DateToYearUpdated).width(Length::Fill).style(theme);
            to_year_column = to_year_column.push(to_year_picklist);

            let mut to_month_column = Column::new().spacing(5).width(Length::FillPortion(1));
            to_month_column = to_month_column.push(Text::new("Month"));
            let to_month_picklist = PickList::new(&mut self.date_to_month_picklist_state, &self.months, Some(self.date_to.month), Message::DateToMonthUpdated).width(Length::Fill).style(theme);
            to_month_column = to_month_column.push(to_month_picklist);
            
            let mut to_day_column = Column::new().spacing(5).width(Length::FillPortion(1));
            to_day_column = to_day_column.push(Text::new("Day"));
            let to_day_picklist = PickList::new(&mut self.date_to_day_picklist_state, &self.days, Some(self.date_to.day), Message::DateToDayUpdated).width(Length::Fill).style(theme);
            to_day_column = to_day_column.push(to_day_picklist);
    
            dates_row = dates_row.push(Text::new("From:").width(Length::Shrink));
            dates_row = dates_row.push(from_year_column);
            dates_row = dates_row.push(from_month_column);
            dates_row = dates_row.push(from_day_column);
            dates_row = dates_row.push(Text::new("To:").width(Length::Shrink));
            dates_row = dates_row.push(to_year_column);
            dates_row = dates_row.push(to_month_column);
            dates_row = dates_row.push(to_day_column);

            main_column = main_column.push(dates_row);
        }

        match self.data {
            Ok(Some(ref data)) => {
                if data.is_empty() {
                    main_column = main_column.push(Text::new("There is no data for this period of time!"));
                } else {
                    let settings = Settings {
                        theme: theme.into(),
                        title: Some(format!("{} to {} graph", self.picked_coin.raw.id, self.picked_currency.raw.name)),
                        min_x_label_distance: DistanceValue::Fixed(160.0),
                        ..Default::default()
                    };
                    let min_x_value = data.iter().map(|(d, _)| *d).min().unwrap();
                    let max_x_value = data.iter().map(|(d, _)| *d).max().unwrap();
                    let min_y_value = data.iter().map(|(_, p)| *p).min_by(|f1, f2| f1.total_cmp(f2)).unwrap();
                    let max_y_value = data.iter().map(|(_, p)| *p).max_by(|f1, f2| f1.total_cmp(f2)).unwrap();
                    let plot_settings = PlotSettings {
                        theme: PlotThemeSettings {
                            line_color: self.settings.read().unwrap().graph_color,
                            point_color: self.settings.read().unwrap().graph_color,
                        },
                        point_size1: 4.0,
                        point_size2: 5.5,
                        point_size3: 7.0,
                        ..Default::default()
                    };
                    let mut plot_data = Vec::new();
                    plot_data.push((plot_settings, data.clone()));
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
            
                    main_column = main_column.push(container_elem.map(Message::ChartMessage));
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

async fn load_data(id: String, vs_currency: String, from: u64, to: u64, timestamp: u64) -> Message {
    let client = coingecko_requests::api_client::Client::new();
    let result = client.market_chart(&id, &vs_currency, from, to)
        .await
        .map(|coin_range| coin_range.prices
            .into_iter()
            .map(|(timestamp, price)| (NaiveDateTime::from_timestamp(timestamp as i64 / 1000, 0), price))
            .collect::<Vec<_>>());
    match result {
        Ok(data) => {
            Message::DataLoaded(data, timestamp)
        }
        Err(err) => {
            Message::DataLoadFailed(err.to_string(), timestamp)
        }
    }
}