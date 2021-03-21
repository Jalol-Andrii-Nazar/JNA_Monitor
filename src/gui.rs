use std::collections::HashMap;

use chrono::{NaiveDate, NaiveDateTime};
use hotplot::chart::line::{self, data::{Message, PlotSettings}};
use iced::{Application, Button, Canvas, Clipboard, Color, Column, Container, Element, Length, PickList, Row, Text, button, pick_list};

pub struct GuiFlags {
    pub ids: Vec<String>,
    pub vs_currencies: Vec<String>,
    pub btc_to_usd: Vec<(NaiveDate, f64)>
}

pub struct Gui {
    ids: Vec<String>,
    vs_currencies: Vec<String>,
    btc_to_usd: Vec<(NaiveDate, f64)>,
    show_menu: bool,
    toggle_menu_button: button::State,
    button1_state: button::State,
    button2_state: button::State,
    button3_state: button::State,
    pls: pick_list::State<String>,
}

#[derive(Debug, Clone)]
pub enum GuiMessage {
    SmthChosen(String),
    ShowMenuToggled,
    Button1Clicked,
    Button2Clicked,
    Button3Clicked,
    ChartMessage(line::data::Message)
}

impl Application for Gui {
    type Executor = iced::executor::Default;
    type Message = GuiMessage;
    type Flags = GuiFlags;

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (Self {
            ids: flags.ids,
            vs_currencies: flags.vs_currencies,
            btc_to_usd: flags.btc_to_usd,
            show_menu: true,
            pls: Default::default(),
            toggle_menu_button: Default::default(),
            button1_state: Default::default(),
            button2_state: Default::default(),
            button3_state: Default::default(),
        }, iced::Command::none())
    }

    fn title(&self) -> String {
        "JNA".to_string()
    }

    fn update(&mut self, message: Self::Message, clipboard: &mut Clipboard) -> iced::Command<Self::Message> {
        match message {
            GuiMessage::SmthChosen(_) => {}
            GuiMessage::ShowMenuToggled => {
                self.show_menu = !self.show_menu;
            }
            GuiMessage::Button1Clicked => {}
            GuiMessage::Button2Clicked => {}
            GuiMessage::Button3Clicked => {}
            GuiMessage::ChartMessage(msg) => {}
        }
        iced::Command::none()
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        let mut top_element = Row::new()
            .width(Length::Fill)
            .height(Length::Fill);
        let mut global_menu = Column::new()
            .padding(5)
            .spacing(5)
            .height(Length::Fill);
        let toggle_menu_button_text = if self.show_menu { Text::new("Hide") } else { Text::new("Show") };
        global_menu = global_menu.push(
            Button::new(&mut self.toggle_menu_button, toggle_menu_button_text)
                .on_press(Self::Message::ShowMenuToggled));
        if self.show_menu {
            global_menu = global_menu.push(
                Button::new(&mut self.button1_state, Text::new("Button1".to_string()))
                    .on_press(Self::Message::Button1Clicked));
            global_menu = global_menu.push(
                Button::new(&mut self.button2_state, Text::new("Button2".to_string()))
                    .on_press(Self::Message::Button2Clicked));
            global_menu = global_menu.push(
                Button::new(&mut self.button3_state, Text::new("Button3".to_string()))
                    .on_press(Self::Message::Button3Clicked));
        }
        top_element = top_element.push(global_menu);
        let picklist = PickList::new(&mut self.pls, &self.vs_currencies, None, Self::Message::SmthChosen);
        top_element = top_element.push(picklist);


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

        top_element = top_element.push(container_elem.map(Self::Message::ChartMessage));

        let mut elem: iced::Element<'_, Self::Message> = top_element.into();
        elem = elem.explain(Color::from_rgb8(0, 0, 0));
        elem
    }
}