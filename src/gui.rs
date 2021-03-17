use iced::{Application, Button, Color, Column, Length, PickList, Row, Text, button, pick_list};

pub struct GuiFlags {
    pub ids: Vec<String>,
    pub vs_currencies: Vec<String>
}

pub struct Gui {
    ids: Vec<String>,
    vs_currencies: Vec<String>,
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
}

impl Application for Gui {
    type Executor = iced::executor::Default;
    type Message = GuiMessage;
    type Flags = GuiFlags;

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (Self {
            ids: flags.ids,
            vs_currencies: flags.vs_currencies,
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

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            GuiMessage::SmthChosen(_) => {}
            GuiMessage::ShowMenuToggled => {
                self.show_menu = !self.show_menu;
            }
            GuiMessage::Button1Clicked => {}
            GuiMessage::Button2Clicked => {}
            GuiMessage::Button3Clicked => {}
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
        let mut elem: iced::Element<'_, Self::Message> = top_element.into();
        elem = elem.explain(Color::from_rgb8(0, 0, 0));
        elem
    }
}