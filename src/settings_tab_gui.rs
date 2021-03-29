use iced::{Clipboard, Command, Text};

pub struct GuiFlags;

#[derive(Default)]
pub struct Gui {

}

impl Gui {
    pub fn new(flags: GuiFlags) -> (Self, Command<GuiMessage>) {
        (Self{}, Command::none())
    }

    pub fn update(&mut self, message: GuiMessage, clipboard: &mut Clipboard) -> Command<GuiMessage> {
        Command::none()
    }

    pub fn view(&mut self) -> iced::Element<'_, GuiMessage> {
        Text::new("Hello Settings").into()
    }
}

#[derive(Debug, Clone)]
pub enum GuiMessage {

}