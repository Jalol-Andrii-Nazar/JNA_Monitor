use std::sync::{Arc, RwLock};

use iced::{Button, Checkbox, Clipboard, Column, Command, Container, Length, PickList, Row, Slider, Text, button, pick_list, slider};

use crate::styling::Theme;

pub struct Flags {
    pub settings: Arc<RwLock<crate::settings::Settings>>
}

#[derive(Debug, Clone)]
pub enum Message {
    ThemeChanged(Theme),
    ShowAllCoinsToggled(bool),
    ShowAllCurrenciesToggled(bool),
    RedChanged(u8),
    GreenChanged(u8),
    BlueChanged(u8),
    AlphaChanged(u8),
    SaveButtonClicked,
}

#[derive(Default)]
pub struct Gui {
    settings: Arc<RwLock<crate::settings::Settings>>,
    theme_pick_list: pick_list::State<Theme>,
    red_slider: slider::State,
    green_slider: slider::State,
    blue_slider: slider::State,
    alpha_slider: slider::State,
    save_button: button::State,
}

impl Gui {
    pub fn new(flags: Flags) -> (Self, Command<Message>) {
        (Self {
            settings: flags.settings,
            theme_pick_list: Default::default(),
            red_slider: Default::default(),
            green_slider: Default::default(),
            blue_slider: Default::default(),
            alpha_slider: Default::default(),
            save_button: Default::default(),
        }, Command::none())
    }

    pub fn update(&mut self, message: Message, _clipboard: &mut Clipboard) -> Command<Message> {
        match message {
            Message::ThemeChanged(theme) => {
                self.settings.write().unwrap().theme = theme;
            }
            Message::ShowAllCoinsToggled(b) => {
                self.settings.write().unwrap().show_all_coins = b;
            }
            Message::ShowAllCurrenciesToggled(b) => {
                self.settings.write().unwrap().show_all_currencies = b;
            }
            Message::RedChanged(red) => {
                self.settings.write().unwrap().graph_color.r = red as f32 / 255.0;
            }
            Message::GreenChanged(green) => {
                self.settings.write().unwrap().graph_color.g = green as f32 / 255.0;
            }
            Message::BlueChanged(blue) => {
                self.settings.write().unwrap().graph_color.b = blue as f32 / 255.0;
            }
            Message::AlphaChanged(alpha) => {
                self.settings.write().unwrap().graph_color.a = alpha as f32 / 255.0;
            }
            Message::SaveButtonClicked => {
                self.settings.read().unwrap().save().unwrap();
            }
        }
        Command::none()
    }

    pub fn view(&mut self) -> iced::Element<'_, Message> {
        let lock = self.settings.read().unwrap();
        let theme = lock.theme;

        let mut column = Column::new()
            .padding(5)
            .width(Length::Fill)
            .height(Length::Fill);

        column = column.push(PickList::new(&mut self.theme_pick_list, Theme::ALL.iter().cloned().collect::<Vec<_>>(), Some(theme.clone()), Message::ThemeChanged).style(theme));
        
        let mut show_all_coins_row = Row::new()
            .padding(5)
            .width(Length::Fill)
            .height(Length::Shrink);
        
        show_all_coins_row = show_all_coins_row.push(Checkbox::new(lock.show_all_coins, "show all coins", Message::ShowAllCoinsToggled).style(theme));
        
        column = column.push(show_all_coins_row);

        let mut show_all_currencies_row = Row::new()
            .padding(5)
            .width(Length::Fill)
            .height(Length::Shrink);
    
        show_all_currencies_row = show_all_currencies_row.push(Checkbox::new(lock.show_all_currencies, "show all currencies", Message::ShowAllCurrenciesToggled).style(theme));
    
        column = column.push(show_all_currencies_row);

        let graph_color = lock.graph_color;
        let red = graph_color.r;
        let green = graph_color.g;
        let blue = graph_color.b;
        let alpha = graph_color.a;

        let mut graph_color_label_row = Row::new()
            .padding(5)
            .spacing(5)
            .width(Length::Fill)
            .height(Length::Shrink);
        
        graph_color_label_row = graph_color_label_row.push(Text::new("Graph color"));
        graph_color_label_row = graph_color_label_row.push(Text::new(format!("{{ r: {:.2}, g: {:.2}, b: {:.2}, a: {:.2}}}", red, green, blue, alpha)).color(graph_color));

        column = column.push(graph_color_label_row);
        
        let mut graph_color_red_row = Row::new()
            .padding(5)
            .width(Length::Fill)
            .height(Length::Shrink);
        
        let mut graph_color_green_row = Row::new()
            .padding(5)
            .width(Length::Fill)
            .height(Length::Shrink);
        
        let mut graph_color_blue_row = Row::new()
            .padding(5)
            .width(Length::Fill)
            .height(Length::Shrink);
        
        let mut graph_color_alpha_row = Row::new()
            .padding(5)
            .width(Length::Fill)
            .height(Length::Shrink);
        
        graph_color_red_row = graph_color_red_row.push(Text::new("Red").width(Length::Units(100)));
        graph_color_red_row = graph_color_red_row.push(Slider::new(&mut self.red_slider, 0..=255, (red * 255.0) as u8, Message::RedChanged).width(Length::Units(256)).style(theme));
        graph_color_green_row = graph_color_green_row.push(Text::new("Green").width(Length::Units(100)));
        graph_color_green_row = graph_color_green_row.push(Slider::new(&mut self.green_slider, 0..=255, (green * 255.0) as u8, Message::GreenChanged).width(Length::Units(256)).style(theme));
        graph_color_blue_row = graph_color_blue_row.push(Text::new("Blue").width(Length::Units(100)));
        graph_color_blue_row = graph_color_blue_row.push(Slider::new(&mut self.blue_slider, 0..=255, (blue * 255.0) as u8, Message::BlueChanged).width(Length::Units(256)).style(theme));
        graph_color_alpha_row = graph_color_alpha_row.push(Text::new("Alpha").width(Length::Units(100)));
        graph_color_alpha_row = graph_color_alpha_row.push(Slider::new(&mut self.alpha_slider, 0..=255, (alpha * 255.0) as u8, Message::AlphaChanged).width(Length::Units(256)).style(theme));
        
        column = column.push(graph_color_red_row);
        column = column.push(graph_color_green_row);
        column = column.push(graph_color_blue_row);
        column = column.push(graph_color_alpha_row);

        column = column.push(Button::new(&mut self.save_button, Text::new("save"))
            .on_press(Message::SaveButtonClicked)
            .style(theme));

        Container::new(column)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(theme)
            .into()
    }
}