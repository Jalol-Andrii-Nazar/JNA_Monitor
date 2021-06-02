use std::{path::PathBuf, rc::Rc, sync::{Arc, RwLock}};

use iced::{Align, Button, Clipboard, Color, Column, Command, Container, Element, Length, Row, Text, button};

use crate::*;

pub struct Flags {
    pub settings: Arc<RwLock<crate::settings::Settings>>,
    pub coins: Rc<Vec<coingecko_requests::data::Coin>>,
    pub currencies: Rc<Vec<coingecko_requests::data::VsCurrency>>,
}

#[derive(Debug, Clone, Copy)]
pub enum Tab {
    Main,
    Triggers,
    Settings,
    About,
}

pub struct Gui {
    settings: Arc<RwLock<crate::settings::Settings>>,
    coins: Rc<Vec<coingecko_requests::data::Coin>>,
    currencies: Rc<Vec<coingecko_requests::data::VsCurrency>>,
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
pub enum Message {
    TabSelected(Tab),
    MainTabMessage(main_tab_gui::Message),
    TriggersTabMessage(triggers_tab_gui::Message),
    SettingsTabMessage(settings_tab_gui::Message),
    AboutTabMessage(about_tab_gui::Message),
}

impl Gui {
    pub fn new(flags: Flags) -> (Self, Command<Message>) {
        let (main_tab_state, main_tab_init_message) = main_tab_gui::Gui::new(main_tab_gui::Flags {
            coins: flags.coins.clone(),
            currencies: flags.currencies.clone(),
            settings: flags.settings.clone(),
        });
        (Self {
            settings: flags.settings,
            coins: flags.coins,
            currencies: flags.currencies,
            active_tab: Tab::Main,
            main_button_state: Default::default(),
            triggers_button_state: Default::default(),
            settings_button_state: Default::default(),
            about_button_state: Default::default(),
            main_tab_state: Some(main_tab_state),
            triggers_tab_state: None,
            settings_tab_state: None,
            about_tab_state: None,
        }, main_tab_init_message.map(Message::MainTabMessage))
    }

    pub fn update(&mut self, message: Message, clipboard: &mut Clipboard) -> Command<Message> {
        match message {
            Message::TabSelected(tab) => {
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
                                let (triggers_tab_state, triggers_tab_init_message) = triggers_tab_gui::Gui::new(triggers_tab_gui::Flags {
                                    coins: self.coins.clone(),
                                    currencies: self.currencies.clone(),
                                    settings: self.settings.clone(),
                                });
                                self.triggers_tab_state = Some(triggers_tab_state);
                                triggers_tab_init_message.map(Message::TriggersTabMessage)
                            }
                        }
                    }
                    Tab::Settings => {
                        match self.settings_tab_state {
                            Some(_) => {
                                Command::none()
                            }
                            None => {
                                let (settings_tab_state, settings_tab_init_message) = settings_tab_gui::Gui::new(settings_tab_gui::Flags {
                                    settings: self.settings.clone(),
                                });
                                self.settings_tab_state = Some(settings_tab_state);
                                settings_tab_init_message.map(Message::SettingsTabMessage)
                            }
                        }
                    }
                    Tab::About => {
                        match self.about_tab_state {
                            Some(_) => {
                                Command::none()
                            }
                            None => {
                                let (about_tab_state, about_tab_init_message) = about_tab_gui::Gui::new(about_tab_gui::Flags {
                                    settings: self.settings.clone(),
                                });
                                self.about_tab_state = Some(about_tab_state);
                                about_tab_init_message.map(Message::AboutTabMessage)
                            }
                        }
                    }
                }
            }
            Message::MainTabMessage(msg) => {
                self.main_tab_state
                    .as_mut()
                    .expect("SHOULD NOT HAPPEN: main tab is initilized in the `new` function!")
                    .update(msg, clipboard)
                    .map(Message::MainTabMessage)
            }
            Message::TriggersTabMessage(msg) => {
                self.triggers_tab_state
                    .as_mut()
                    .expect("SHOULD NOT HAPPEN: a message for triggers tab cannot be received before it is initialized!")
                    .update(msg, clipboard)
                    .map(Message::TriggersTabMessage)
            }
            Message::SettingsTabMessage(msg) => {
                self.settings_tab_state
                    .as_mut()
                    .expect("SHOULD NOT HAPPEN: a message for settings tab cannot be received before it is initialized!")
                    .update(msg, clipboard)
                    .map(Message::SettingsTabMessage)
            }
            Message::AboutTabMessage(msg) => {
                self.about_tab_state
                    .as_mut()
                    .expect("SHOULD NOT HAPPEN: a message for about tab cannot be received before it is initialized!")
                    .update(msg, clipboard)
                    .map(Message::AboutTabMessage)
            }
        }
    }

    pub fn view(&mut self) -> iced::Element<'_, Message> {
        let lock = self.settings.write().unwrap();
        let theme = lock.theme.clone();
        drop(lock);

        let mut global_menu = Column::new()
            .spacing(5)
            .align_items(Align::Start)
            .width(Length::Shrink)
            .max_width(100)
            .height(Length::Fill);
        global_menu = global_menu.push(
            Button::new(&mut self.main_button_state, Text::new("Main".to_string()))
                .on_press(Message::TabSelected(Tab::Main))
                .width(Length::Units(100))
                .style(theme));
        global_menu = global_menu.push(
            Button::new(&mut self.triggers_button_state, Text::new("Triggers".to_string()))
                .on_press(Message::TabSelected(Tab::Triggers))
                .width(Length::Units(100))
                .style(theme));
        global_menu = global_menu.push(
            Button::new(&mut self.settings_button_state, Text::new("Settings".to_string()))
                .on_press(Message::TabSelected(Tab::Settings))
                .width(Length::Units(100))
                .style(theme));
        global_menu = global_menu.push(
            Button::new(&mut self.about_button_state, Text::new("About".to_string()))
                .on_press(Message::TabSelected(Tab::About))
                .width(Length::Units(100))
                .style(theme));

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
                    .map(Message::MainTabMessage));
            }
            Tab::Triggers => {
                element = element.push(self.triggers_tab_state
                    .as_mut()
                    .expect("SHOULD NOT HAPPEN: triggers tab cannot be selected before it is initialized!")
                    .view()
                    .map(Message::TriggersTabMessage));
            }
            Tab::Settings => {
                element = element.push(self.settings_tab_state
                    .as_mut()
                    .expect("SHOULD NOT HAPPEN: settings tab cannot be selected before it is initialized!")
                    .view()
                    .map(Message::SettingsTabMessage));
            }
            Tab::About => {
                element = element.push(self.about_tab_state
                    .as_mut()
                    .expect("SHOULD NOT HAPPEN: about tab cannot be selected before it is initialized!")
                    .view()
                    .map(Message::AboutTabMessage));
            }
        }

        Container::new(element)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(theme)
            .into()
    }
}