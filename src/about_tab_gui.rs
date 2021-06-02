use std::path::PathBuf;

use iced::{Button, Clipboard, Row, Column, Command, Length, Scrollable, Text, button, scrollable, HorizontalAlignment, VerticalAlignment};
use std::sync::{Arc, RwLock};
use iced::widget::image::Image;
use iced::widget::image::Handle;

const TUTORIAL: [&[u8]; 4] = [
    include_bytes!("../tutorial1.png"),
    include_bytes!("../tutorial2.png"),
    include_bytes!("../tutorial3.png"),
    include_bytes!("../tutorial4.png")
];

pub struct Flags {
    pub settings: Arc<RwLock<crate::settings::Settings>>
}

#[derive(Debug, Clone)]
pub enum Message {
}

pub struct Gui {
    settings: Arc<RwLock<crate::settings::Settings>>,
    scrollable_state: scrollable::State,
    tutorial_button_state: button::State,
}

impl Gui {
    pub fn new(flags: Flags) -> (Self, Command<Message>) {
        (Self {
            settings: flags.settings,
            scrollable_state: Default::default(),
            tutorial_button_state: Default::default(),
        }, Command::none())
    }

    pub fn update(&mut self, message: Message, _clipboard: &mut Clipboard) -> Command<Message> {
        match message {
            _ => {

            }
        }
        Command::none()
    }

    pub fn view(&mut self) -> iced::Element<'_, Message> {
        let lock = self.settings.read().unwrap();
        let theme = lock.theme;

        let mut element = Column::new()
            .padding(5)
            .spacing(5)
            .width(Length::Fill)
            .height(Length::Fill);
        
        let mut scrollable = Scrollable::new(&mut self.scrollable_state)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(theme);
        
        element = element.push(
            Text::new("User guide")
                .width(Length::Fill)
                .height(Length::Shrink)
                .size(32)
                .horizontal_alignment(HorizontalAlignment::Center)
        );

        for image_bytes in TUTORIAL {
            let image = Image::new(Handle::from_memory(image_bytes.iter().copied().collect()))
                .width(Length::Fill);
            scrollable = scrollable.push(image);
        }
        
        element = element.push(scrollable);

        let mut authors_row = Row::new()
            .padding(5)
            .spacing(5)
            .width(Length::Fill);
        
        element = element.push(
            Text::new("Authors: K. Jalol, K. Nazar, T. Andrii")
                .width(Length::Fill)
                .size(14)
                .horizontal_alignment(HorizontalAlignment::Center)
        );

        element = element.push(authors_row);

        element.into()
    }
}