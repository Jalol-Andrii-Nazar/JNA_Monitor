use std::collections::HashMap;

use chrono::NaiveDate;
use hotplot::chart::line::{self, data::PlotSettings};
use iced::{Canvas, Clipboard, Column, Command, Container, Element, Length, PickList, Row, Text, pick_list};

pub struct GuiFlags {
    pub ids: Vec<String>,
    pub vs_currencies: Vec<String>,
    pub btc_to_usd: Vec<(NaiveDate, f64)>
}

pub struct Gui {
    ids: Vec<String>,
    vs_currencies: Vec<String>,
    btc_to_usd: Vec<(NaiveDate, f64)>,
    picked_id: String,
    picked_vs_currency: String,
    id_picklist_state: pick_list::State<String>,
    vs_currency_picklist_state: pick_list::State<String>,
}

#[derive(Debug, Clone)]
pub enum GuiMessage {
    IdPicked(String),
    VsCurrencyPicked(String),
    ChartMessage(line::data::Message)
}

impl Gui {
    pub fn new(flags: GuiFlags) -> (Self, Command<GuiMessage>) {
        let picked_id = flags.ids[0].clone();
        let picked_vs_currency = flags.vs_currencies[0].clone();
        (Self {
            ids: flags.ids,
            vs_currencies: flags.vs_currencies,
            btc_to_usd: flags.btc_to_usd,
            picked_id,
            picked_vs_currency,
            id_picklist_state: Default::default(),
            vs_currency_picklist_state: Default::default(),
        }, iced::Command::none())
    }

    pub fn update(&mut self, message: GuiMessage, clipboard: &mut Clipboard) -> iced::Command<GuiMessage> {
        match message {
            GuiMessage::IdPicked(picked) => {
                self.picked_id = picked;
            }
            GuiMessage::VsCurrencyPicked(picked) => {
                self.picked_vs_currency = picked;
            }
            GuiMessage::ChartMessage(msg) => {}
        }
        iced::Command::none()
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

        let settings = line::data::Settings {
            ..Default::default()
        };
        let min_x_value = self.btc_to_usd.iter().map(|(d, _)| *d).min().unwrap();
        let max_x_value = self.btc_to_usd.iter().map(|(d, _)| *d).max().unwrap();
        let min_y_value = self.btc_to_usd.iter().map(|(_, p)| *p).min_by(|f1, f2| f1.total_cmp(f2)).unwrap();
        let max_y_value = self.btc_to_usd.iter().map(|(_, p)| *p).max_by(|f1, f2| f1.total_cmp(f2)).unwrap();
        let plot_settings = PlotSettings {
            ..Default::default()
        };
        let mut data = HashMap::new();
        data.insert(plot_settings, self.btc_to_usd.clone());
        let chart = line::Chart::new(
            settings,
            min_x_value,
            max_x_value,
            min_y_value,
            max_y_value,
            data,
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

        main_column.into()
    }
}