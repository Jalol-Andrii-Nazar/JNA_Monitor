use std::collections::HashMap;

use chrono::{NaiveDate, NaiveDateTime};
use hotplot::chart::line::{self, data::PlotSettings};
use iced::{Canvas, Clipboard, Column, Command, Container, Element, Length, PickList, Row, Text, pick_list};

pub struct GuiFlags {
    pub ids: Vec<String>,
    pub vs_currencies: Vec<String>,
}

pub struct Gui {
    ids: Vec<String>,
    vs_currencies: Vec<String>,
    data: Result<Option<Vec<(NaiveDate, f64)>>, Box<dyn std::error::Error>>,
    picked_id: String,
    picked_vs_currency: String,
    id_picklist_state: pick_list::State<String>,
    vs_currency_picklist_state: pick_list::State<String>,
}

#[derive(Debug, Clone)]
pub enum GuiMessage {
    IdPicked(String),
    VsCurrencyPicked(String),
    DataLoaded(Vec<(NaiveDate, f64)>),
    DataLoadFailed(String),
    ChartMessage(line::data::Message)
}

impl Gui {
    pub fn new(flags: GuiFlags) -> (Self, Command<GuiMessage>) {
        let picked_id = flags.ids[0].clone();
        let picked_vs_currency = flags.vs_currencies[0].clone();
        (Self {
            ids: flags.ids,
            vs_currencies: flags.vs_currencies,
            data: Ok(None),
            picked_id: picked_id.clone(),
            picked_vs_currency: picked_vs_currency.clone(),
            id_picklist_state: Default::default(),
            vs_currency_picklist_state: Default::default(),
        }, Command::perform(load_data(picked_id.clone(), picked_vs_currency.clone()), |x| x))
    }

    pub fn update(&mut self, message: GuiMessage, clipboard: &mut Clipboard) -> Command<GuiMessage> {
        match message {
            GuiMessage::IdPicked(picked) => {
                self.picked_id = picked;
                self.data = Ok(None);
                Command::perform(load_data(self.picked_id.clone(), self.picked_vs_currency.clone()), |x| x)
            }
            GuiMessage::VsCurrencyPicked(picked) => {
                self.picked_vs_currency = picked;
                self.data = Ok(None);
                Command::perform(load_data(self.picked_id.clone(), self.picked_vs_currency.clone()), |x| x)
            }
            GuiMessage::DataLoaded(data) => {
                self.data = Ok(Some(data));
                Command::none()
            }
            GuiMessage::DataLoadFailed(err) => {
                self.data = Err(err.into());
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

        let mut id_column = Column::new().spacing(5).width(Length::Units(200));
        id_column = id_column.push(Text::new("Cryptocurrency id"));
        let id_picklist = PickList::new(&mut self.id_picklist_state, &self.ids, Some(self.picked_id.clone()), GuiMessage::IdPicked).width(Length::Fill);
        id_column = id_column.push(id_picklist);

        let mut vs_currency_column = Column::new().spacing(5).width(Length::Units(200));
        vs_currency_column = vs_currency_column.push(Text::new("Vs Currency"));
        let vs_currency_picklist = PickList::new(&mut self.vs_currency_picklist_state, &self.vs_currencies, Some(self.picked_vs_currency.clone()), GuiMessage::VsCurrencyPicked).width(Length::Fill);
        vs_currency_column = vs_currency_column.push(vs_currency_picklist);
        
        chart_settings_row = chart_settings_row.push(id_column);
        chart_settings_row = chart_settings_row.push(vs_currency_column);

        main_column = main_column.push(chart_settings_row);

        match self.data {
            Ok(Some(ref data)) => {
                if data.is_empty() {
                    main_column = main_column.push(Text::new("There is no data for this period of time!"));
                } else {
                    let settings = line::data::Settings {
                        ..Default::default()
                    };
                    let min_x_value = data.iter().map(|(d, _)| *d).min().unwrap();
                    let max_x_value = data.iter().map(|(d, _)| *d).max().unwrap();
                    let min_y_value = data.iter().map(|(_, p)| *p).min_by(|f1, f2| f1.total_cmp(f2)).unwrap();
                    let max_y_value = data.iter().map(|(_, p)| *p).max_by(|f1, f2| f1.total_cmp(f2)).unwrap();
                    let plot_settings = PlotSettings {
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
                        plot_data,
                        Vec::new(),
                        Vec::new()
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

async fn load_data(id: String, vs_currency: String) -> GuiMessage {
    let client = coingecko_requests::client::Client::new();
    let result = client.coins_id_market_chart_range(&id, &vs_currency, 1392577232, 1422577232)
        .await
        .map(|coin_range| coin_range.prices
            .into_iter()
            .map(|(timestamp, price)| (NaiveDateTime::from_timestamp(timestamp as i64 / 1000, 0).date(), price))
            .collect::<Vec<_>>());
    match result {
        Ok(data) => {
            GuiMessage::DataLoaded(data)
        }
        Err(err) => {
            GuiMessage::DataLoadFailed(err.to_string())
        }
    }
}