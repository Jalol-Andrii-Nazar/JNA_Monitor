use std::{char, collections::HashMap};

use chrono::NaiveDate;
use hotplot::chart::line::{self, data::PlotSettings};
use iced::{Align, Application, Button, Canvas, Clipboard, Color, Column, Container, Element, Length, PickList, Row, Text, button, pick_list};

use crate::main;

pub struct GuiFlags {
    pub ids: Vec<String>,
    pub vs_currencies: Vec<String>,
    pub btc_to_usd: Vec<(NaiveDate, f64)>
}

#[derive(Debug, Clone, Copy)]
enum Tab {
    Main,
    Triggers,
    Settings
}

pub struct Gui {
    ids: Vec<String>,
    vs_currencies: Vec<String>,
    btc_to_usd: Vec<(NaiveDate, f64)>,
    picked_id: String,
    picked_vs_currency: String,
    active_tab: Tab,
    main_button_state: button::State,
    triggers_button_state: button::State,
    settings_button_state: button::State,
    id_picklist_state: pick_list::State<String>,
    vs_currency_picklist_state: pick_list::State<String>,
}

#[derive(Debug, Clone)]
pub enum GuiMessage {
    MainButtonClicked,
    TriggersButtonClicked,
    SettingsButtonClicked,
    IdPicked(String),
    VsCurrencyPicked(String),
    ChartMessage(line::data::Message)
}

impl Application for Gui {
    type Executor = iced::executor::Default;
    type Message = GuiMessage;
    type Flags = GuiFlags;

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let picked_id = flags.ids[0].clone();
        let picked_vs_currency = flags.vs_currencies[0].clone();
        (Self {
            ids: flags.ids,
            vs_currencies: flags.vs_currencies,
            btc_to_usd: flags.btc_to_usd,
            picked_id,
            picked_vs_currency,
            active_tab: Tab::Main,
            main_button_state: Default::default(),
            triggers_button_state: Default::default(),
            settings_button_state: Default::default(),
            id_picklist_state: Default::default(),
            vs_currency_picklist_state: Default::default(),
        }, iced::Command::none())
    }

    fn title(&self) -> String {
        "JNA".to_string()
    }

    fn update(&mut self, message: Self::Message, clipboard: &mut Clipboard) -> iced::Command<Self::Message> {
        match message {
            GuiMessage::IdPicked(picked) => {
                self.picked_id = picked;
            }
            GuiMessage::VsCurrencyPicked(picked) => {
                self.picked_vs_currency = picked;
            }
            GuiMessage::MainButtonClicked => {
                self.active_tab = Tab::Main;
            }
            GuiMessage::TriggersButtonClicked => {
                self.active_tab = Tab::Triggers;
            }
            GuiMessage::SettingsButtonClicked => {
                self.active_tab = Tab::Settings;
            }
            GuiMessage::ChartMessage(msg) => {}
        }
        iced::Command::none()
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        let mut global_menu = Column::new()
            .spacing(5)
            .align_items(Align::Start)
            .width(Length::Shrink)
            .max_width(100)
            .height(Length::Fill);
        global_menu = global_menu.push(
            Button::new(&mut self.main_button_state, Text::new("Main".to_string()))
                .on_press(Self::Message::MainButtonClicked))
                .width(Length::Units(100));
        global_menu = global_menu.push(
            Button::new(&mut self.triggers_button_state, Text::new("Triggers".to_string()))
                .on_press(Self::Message::TriggersButtonClicked))
                .width(Length::Units(100));
        global_menu = global_menu.push(
            Button::new(&mut self.settings_button_state, Text::new("Settings".to_string()))
                .on_press(Self::Message::SettingsButtonClicked))
                .width(Length::Units(100));
        
        let mut main_column = Column::new().spacing(5);

        let mut chart_settings_row = Row::new().spacing(5).width(Length::Shrink);

        let mut id_column = Column::new().spacing(5).width(Length::Units(200));
        id_column = id_column.push(Text::new("Cryptocurrency id"));
        let id_picklist = PickList::new(&mut self.id_picklist_state, &self.ids, Some(self.picked_id.clone()), Self::Message::IdPicked).width(Length::Fill);
        id_column = id_column.push(id_picklist);

        let mut vs_currency_column = Column::new().spacing(5).width(Length::Units(200));
        vs_currency_column = vs_currency_column.push(Text::new("Vs Currency"));
        let vs_currency_picklist = PickList::new(&mut self.vs_currency_picklist_state, &self.vs_currencies, Some(self.picked_vs_currency.clone()), Self::Message::VsCurrencyPicked).width(Length::Fill);
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

        main_column = main_column.push(container_elem.map(Self::Message::ChartMessage));

        // top_element = top_element.push(container_elem.map(Self::Message::ChartMessage));

        let mut element = Row::new()
            .padding(2)
            .spacing(5)
            .align_items(Align::Start)
            .width(Length::Fill)
            .height(Length::Fill)
            .push(global_menu);

        element = element.push(main_column);

        let mut menu: Element<Self::Message> = element.into();
        //if self.debug {
            menu = menu.explain(Color::BLACK);
        //}
        menu
    }
}