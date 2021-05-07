use iced::{Align, Button, Clipboard, Color, Column, Command, Element, Length, Row, Text, button};

use crate::*;

const ENABLE_EXPLAIN: bool = false;

pub struct GuiFlags {
    pub coins: Vec<coingecko_requests::data::RawCoin>,
    pub vs_currencies: Vec<coingecko_requests::data::RawVsCurrency>,
}

#[derive(Debug, Clone, Copy)]
pub enum Tab {
    Main,
    Triggers,
    Settings,
    About,
}

pub struct Gui {
    active_tab: Tab,
    main_button_state: button::State,
    triggers_button_state: button::State,
    settings_button_state: button::State,
    about_button_state: button::State,
    main_tab_state: Option<main_tab_gui::Gui>,
    triggers_tab_state: Option<triggers_tab_gui::Gui>,
    settings_tab_state: Option<settings_tab_gui::Gui>,
    about_tab_state: Option<about_tab_gui::Gui>,
}

#[derive(Debug, Clone)]
pub enum GuiMessage {
    TabSelected(Tab),
    MainTabMessage(main_tab_gui::GuiMessage),
    TriggersTabMessage(triggers_tab_gui::GuiMessage),
    SettingsTabMessage(settings_tab_gui::GuiMessage),
    AboutTabMessage(about_tab_gui::GuiMessage),
}

impl Gui {
    pub fn new(flags: GuiFlags) -> (Self, Command<GuiMessage>) {
        let (main_tab_state, main_tab_init_message) = main_tab_gui::Gui::new(main_tab_gui::GuiFlags {
            coins: flags.coins,
            vs_currencies: flags.vs_currencies
        });
        (Self {
            active_tab: Tab::Main,
            main_button_state: Default::default(),
            triggers_button_state: Default::default(),
            settings_button_state: Default::default(),
            about_button_state: Default::default(),
            main_tab_state: Some(main_tab_state),
            triggers_tab_state: None,
            settings_tab_state: None,
            about_tab_state: None,
        }, main_tab_init_message.map(GuiMessage::MainTabMessage))
    }

    pub fn update(&mut self, message: GuiMessage, clipboard: &mut Clipboard) -> Command<GuiMessage> {
        match message {
            GuiMessage::TabSelected(tab) => {
                self.active_tab = tab;
                match tab {
                    Tab::Main => {
                        let _ = self.main_tab_state.as_ref().expect("SHOULD NOT HAPPEN: main tab is initilized in the `new` function!");
                        Command::none()
                    }
                    Tab::Triggers => {
                        match self.triggers_tab_state {
                            Some(_) => {
                                Command::none()
                            }
                            None => {
                                let (triggers_tab_state, triggers_tab_init_message) = triggers_tab_gui::Gui::new(triggers_tab_gui::GuiFlags {});
                                self.triggers_tab_state = Some(triggers_tab_state);
                                triggers_tab_init_message.map(GuiMessage::TriggersTabMessage)
                            }
                        }
                    }
                    Tab::Settings => {
                        match self.settings_tab_state {
                            Some(_) => {
                                Command::none()
                            }
                            None => {
                                let (settings_tab_state, settings_tab_init_message) = settings_tab_gui::Gui::new(settings_tab_gui::GuiFlags {});
                                self.settings_tab_state = Some(settings_tab_state);
                                settings_tab_init_message.map(GuiMessage::SettingsTabMessage)
                            }
                        }
                    }
                    Tab::About => {
                        match self.about_tab_state {
                            Some(_) => {
                                Command::none()
                            }
                            None => {
                                let (about_tab_state, about_tab_init_message) = about_tab_gui::Gui::new(about_tab_gui::GuiFlags {});
                                self.about_tab_state = Some(about_tab_state);
                                about_tab_init_message.map(GuiMessage::AboutTabMessage)
                            }
                        }
                    }
                }
            }
            GuiMessage::MainTabMessage(msg) => {
                self.main_tab_state
                    .as_mut()
                    .expect("SHOULD NOT HAPPEN: main tab is initilized in the `new` function!")
                    .update(msg, clipboard)
                    .map(GuiMessage::MainTabMessage)
            }
            GuiMessage::TriggersTabMessage(msg) => {
                self.triggers_tab_state
                    .as_mut()
                    .expect("SHOULD NOT HAPPEN: a message for triggers tab cannot be received before it is initialized!")
                    .update(msg, clipboard)
                    .map(GuiMessage::TriggersTabMessage)
            }
            GuiMessage::SettingsTabMessage(msg) => {
                self.settings_tab_state
                    .as_mut()
                    .expect("SHOULD NOT HAPPEN: a message for settings tab cannot be received before it is initialized!")
                    .update(msg, clipboard)
                    .map(GuiMessage::SettingsTabMessage)
            }
            GuiMessage::AboutTabMessage(msg) => {
                self.about_tab_state
                    .as_mut()
                    .expect("SHOULD NOT HAPPEN: a message for about tab cannot be received before it is initialized!")
                    .update(msg, clipboard)
                    .map(GuiMessage::AboutTabMessage)
            }
        }
    }

    pub fn view(&mut self) -> iced::Element<'_, GuiMessage> {
        let mut global_menu = Column::new()
            .spacing(5)
            .align_items(Align::Start)
            .width(Length::Shrink)
            .max_width(100)
            .height(Length::Fill);
        global_menu = global_menu.push(
            Button::new(&mut self.main_button_state, Text::new("Main".to_string()))
                .on_press(GuiMessage::TabSelected(Tab::Main))
                .width(Length::Units(100)));
        global_menu = global_menu.push(
            Button::new(&mut self.triggers_button_state, Text::new("Triggers".to_string()))
                .on_press(GuiMessage::TabSelected(Tab::Triggers))
                .width(Length::Units(100)));
        global_menu = global_menu.push(
            Button::new(&mut self.settings_button_state, Text::new("Settings".to_string()))
                .on_press(GuiMessage::TabSelected(Tab::Settings))
                .width(Length::Units(100)));
        global_menu = global_menu.push(
            Button::new(&mut self.about_button_state, Text::new("About".to_string()))
                .on_press(GuiMessage::TabSelected(Tab::About))
                .width(Length::Units(100)));

        let mut element = Row::new()
            .padding(2)
            .spacing(5)
            .align_items(Align::Start)
            .width(Length::Fill)
            .height(Length::Fill)
            .push(global_menu);

        match self.active_tab {
            Tab::Main => {
                element = element.push(self.main_tab_state
                    .as_mut()
                    .expect("SHOULD NOT HAPPEN: main tab cannot be selected before it is initialized!")
                    .view()
                    .map(GuiMessage::MainTabMessage));
            }
            Tab::Triggers => {
                element = element.push(self.triggers_tab_state
                    .as_mut()
                    .expect("SHOULD NOT HAPPEN: triggers tab cannot be selected before it is initialized!")
                    .view()
                    .map(GuiMessage::TriggersTabMessage));
            }
            Tab::Settings => {
                element = element.push(self.settings_tab_state
                    .as_mut()
                    .expect("SHOULD NOT HAPPEN: settings tab cannot be selected before it is initialized!")
                    .view()
                    .map(GuiMessage::SettingsTabMessage));
            }
            Tab::About => {
                element = element.push(self.about_tab_state
                    .as_mut()
                    .expect("SHOULD NOT HAPPEN: about tab cannot be selected before it is initialized!")
                    .view()
                    .map(GuiMessage::AboutTabMessage));
            }
        }

        let mut menu: Element<GuiMessage> = element.into();
        if ENABLE_EXPLAIN {
            menu = menu.explain(Color::BLACK);
        }
        menu
    }
}