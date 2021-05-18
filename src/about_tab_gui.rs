use iced::{Clipboard, Command, Text};

pub struct Flags;

#[derive(Debug, Clone)]
pub enum Message {

}

#[derive(Default)]
pub struct Gui {

}

impl Gui {
    pub fn new(flags: Flags) -> (Self, Command<Message>) {
        (Self{}, Command::none())
    }

    pub fn update(&mut self, message: Message, _clipboard: &mut Clipboard) -> Command<Message> {
        Command::none()
    }

    pub fn view(&mut self) -> iced::Element<'_, Message> {
        Text::new("Hello About").into()
    }
}