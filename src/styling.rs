use std::fmt::Display;

use hotplot::chart::line::data::ThemeSettings;
use iced::{button, checkbox, container, pick_list, progress_bar, radio, rule, scrollable, slider, text_input, Color};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    Default = 0,
    Light = 1,
    Dark = 2,
}

impl Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Theme::Default => { write!(f, "Default") }
            Theme::Light => { write!(f, "Light") }
            Theme::Dark => { write!(f, "Dark") }
        }
    }
}

impl Theme {
    pub const ALL: [Theme; 3] = [Theme::Default, Theme::Light, Theme::Dark];

    pub fn from_discriminant(discriminant: u8) -> Option<Self> {
        match discriminant {
            0 => Some(Theme::Default),
            1 => Some(Theme::Light),
            2 => Some(Theme::Dark),
            _ => None
        }
    }
}

impl Default for Theme {
    fn default() -> Theme {
        Theme::Default
    }
}

impl From<Theme> for ThemeSettings {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Default | Theme::Light => {
                ThemeSettings {
                    background_color: Color::from_rgb8(211, 211, 211),
                    padded_background_color: Color::WHITE,
                    margined_background_color: Some(Color::from_rgb8(241, 241, 241)),
                    title_color: Color::BLACK,
                    title_size: 32.0,
                }
            }
            Theme::Dark => {
                ThemeSettings {
                    background_color: Color::from_rgb8(0x36, 0x39, 0x3F),
                    padded_background_color: Color::from_rgb8(0x36, 0x39, 0x3F),
                    margined_background_color: Some(dark::SURFACE),
                    title_color: Color::WHITE,
                    title_size: 32.0,
                }
            }
        }
    }
}

impl From<Theme> for Box<dyn container::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Default => Default::default(),
            Theme::Light => Default::default(),
            Theme::Dark => dark::Container.into(),
        }
    }
}

impl From<Theme> for Box<dyn radio::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Default => Default::default(),
            Theme::Light => Default::default(),
            Theme::Dark => dark::Radio.into(),
        }
    }
}

impl From<Theme> for Box<dyn text_input::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Default => Default::default(),
            Theme::Light => Default::default(),
            Theme::Dark => dark::TextInput.into(),
        }
    }
}

impl From<Theme> for Box<dyn button::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Default => Default::default(),
            Theme::Light => light::Button.into(),
            Theme::Dark => dark::Button.into(),
        }
    }
}

impl From<Theme> for Box<dyn scrollable::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Default => Default::default(),
            Theme::Light => Default::default(),
            Theme::Dark => dark::Scrollable.into(),
        }
    }
}

impl From<Theme> for Box<dyn slider::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Default => Default::default(),
            Theme::Light => Default::default(),
            Theme::Dark => dark::Slider.into(),
        }
    }
}

impl From<Theme> for Box<dyn progress_bar::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Default => Default::default(),
            Theme::Light => Default::default(),
            Theme::Dark => dark::ProgressBar.into(),
        }
    }
}

impl From<Theme> for Box<dyn checkbox::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Default => Default::default(),
            Theme::Light => light::Checkbox.into(),
            Theme::Dark => dark::Checkbox.into(),
        }
    }
}

impl From<Theme> for Box<dyn rule::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Default => Default::default(),
            Theme::Light => Default::default(),
            Theme::Dark => dark::Rule.into(),
        }
    }
}

impl From<Theme> for Box<dyn pick_list::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Default => Default::default(),
            Theme::Light => light::PickList.into(),
            Theme::Dark => dark::PickList.into(),
        }
    }
}

mod light {
    use iced::{Color, Vector, button, checkbox, pick_list};

    pub struct Button;

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            button::Style {
                background: Color::from_rgb(0.11, 0.42, 0.87).into(),
                border_radius: 12.0,
                shadow_offset: Vector::new(1.0, 1.0),
                text_color: Color::from_rgb8(0xEE, 0xEE, 0xEE),
                ..button::Style::default()
            }
        }

        fn hovered(&self) -> button::Style {
            button::Style {
                text_color: Color::WHITE,
                shadow_offset: Vector::new(1.0, 2.0),
                ..self.active()
            }
        }
    }

    pub struct PickList;

    impl pick_list::StyleSheet for PickList {
        fn menu(&self) -> pick_list::Menu {
            pick_list::Menu {
                background: Color::from_rgb(0.11, 0.42, 0.87).into(),
                text_color: Color::from_rgb8(0xEE, 0xEE, 0xEE),
                ..Default::default()
            }
        }

        fn active(&self) -> pick_list::Style {
            pick_list::Style {
                background: Color::from_rgb(0.11, 0.42, 0.87).into(),
                border_radius: 12.0,
                text_color: Color::from_rgb8(0xEE, 0xEE, 0xEE),
                ..Default::default()
            }
        }

        fn hovered(&self) -> pick_list::Style {
            pick_list::Style {
                background: Color::from_rgb(0.11, 0.42, 0.87).into(),
                border_radius: 12.0,
                text_color: Color::from_rgb8(0xEE, 0xEE, 0xEE),
                ..Default::default()
            }
        }
    }

    pub struct Checkbox;

    impl checkbox::StyleSheet for Checkbox {
        fn active(&self, is_checked: bool) -> checkbox::Style {
            checkbox::Style {
                background: if is_checked { Color::from_rgb(0.11, 0.42, 0.87).into() } else { Color::from_rgb(0.4, 0.4, 0.4).into() },
                checkmark_color: Color::from_rgb8(0xEE, 0xEE, 0xEE).into(),
                border_radius: 12.0,
                border_width: 1.0,
                border_color: Color::BLACK
            }
        }

        fn hovered(&self, is_checked: bool) -> checkbox::Style {
            checkbox::Style {
                background: if is_checked { Color::from_rgb(0.11, 0.42, 0.87).into() } else { Color::from_rgb(0.4, 0.4, 0.4).into() },
                checkmark_color: Color::from_rgb8(0xEE, 0xEE, 0xEE).into(),
                border_radius: 12.0,
                border_width: 1.0,
                border_color: Color::BLACK
            }
        }
    }
}

mod dark {
    use iced::{Background, Color, button, checkbox, container, pick_list, progress_bar, radio, rule, scrollable, slider, text_input};

    pub const SURFACE: Color = Color::from_rgb(
        0x40 as f32 / 255.0,
        0x44 as f32 / 255.0,
        0x4B as f32 / 255.0,
    );

    pub const ACCENT: Color = Color::from_rgb(
        0x6F as f32 / 255.0,
        0xFF as f32 / 255.0,
        0xE9 as f32 / 255.0,
    );

    pub const ACTIVE: Color = Color::from_rgb(
        0x72 as f32 / 255.0,
        0x89 as f32 / 255.0,
        0xDA as f32 / 255.0,
    );

    pub const HOVERED: Color = Color::from_rgb(
        0x67 as f32 / 255.0,
        0x7B as f32 / 255.0,
        0xC4 as f32 / 255.0,
    );

    pub struct Container;

    impl container::StyleSheet for Container {
        fn style(&self) -> container::Style {
            container::Style {
                background: Color::from_rgb8(0x36, 0x39, 0x3F).into(),
                text_color: Color::WHITE.into(),
                ..container::Style::default()
            }
        }
    }

    pub struct Radio;

    impl radio::StyleSheet for Radio {
        fn active(&self) -> radio::Style {
            radio::Style {
                background: SURFACE.into(),
                dot_color: ACTIVE,
                border_width: 1.0,
                border_color: ACTIVE,
            }
        }

        fn hovered(&self) -> radio::Style {
            radio::Style {
                background: Color { a: 0.5, ..SURFACE }.into(),
                ..self.active()
            }
        }
    }

    pub struct TextInput;

    impl text_input::StyleSheet for TextInput {
        fn active(&self) -> text_input::Style {
            text_input::Style {
                background: SURFACE.into(),
                border_radius: 2.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            }
        }

        fn focused(&self) -> text_input::Style {
            text_input::Style {
                border_width: 1.0,
                border_color: ACCENT,
                ..self.active()
            }
        }

        fn hovered(&self) -> text_input::Style {
            text_input::Style {
                border_width: 1.0,
                border_color: Color { a: 0.3, ..ACCENT },
                ..self.focused()
            }
        }

        fn placeholder_color(&self) -> Color {
            Color::from_rgb(0.4, 0.4, 0.4)
        }

        fn value_color(&self) -> Color {
            Color::WHITE
        }

        fn selection_color(&self) -> Color {
            ACTIVE
        }
    }

    pub struct Button;

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            button::Style {
                background: ACTIVE.into(),
                border_radius: 3.0,
                text_color: Color::WHITE,
                ..button::Style::default()
            }
        }

        fn hovered(&self) -> button::Style {
            button::Style {
                background: HOVERED.into(),
                text_color: Color::WHITE,
                ..self.active()
            }
        }

        fn pressed(&self) -> button::Style {
            button::Style {
                border_width: 1.0,
                border_color: Color::WHITE,
                ..self.hovered()
            }
        }
    }

    pub struct Scrollable;

    impl scrollable::StyleSheet for Scrollable {
        fn active(&self) -> scrollable::Scrollbar {
            scrollable::Scrollbar {
                background: SURFACE.into(),
                border_radius: 2.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
                scroller: scrollable::Scroller {
                    color: ACTIVE,
                    border_radius: 2.0,
                    border_width: 0.0,
                    border_color: Color::TRANSPARENT,
                },
            }
        }

        fn hovered(&self) -> scrollable::Scrollbar {
            let active = self.active();

            scrollable::Scrollbar {
                background: Color { a: 0.5, ..SURFACE }.into(),
                scroller: scrollable::Scroller {
                    color: HOVERED,
                    ..active.scroller
                },
                ..active
            }
        }

        fn dragging(&self) -> scrollable::Scrollbar {
            let hovered = self.hovered();

            scrollable::Scrollbar {
                scroller: scrollable::Scroller {
                    color: Color::from_rgb(0.85, 0.85, 0.85),
                    ..hovered.scroller
                },
                ..hovered
            }
        }
    }

    pub struct Slider;

    impl slider::StyleSheet for Slider {
        fn active(&self) -> slider::Style {
            slider::Style {
                rail_colors: (ACTIVE, Color { a: 0.1, ..ACTIVE }),
                handle: slider::Handle {
                    shape: slider::HandleShape::Circle { radius: 9.0 },
                    color: ACTIVE,
                    border_width: 0.0,
                    border_color: Color::TRANSPARENT,
                },
            }
        }

        fn hovered(&self) -> slider::Style {
            let active = self.active();

            slider::Style {
                handle: slider::Handle {
                    color: HOVERED,
                    ..active.handle
                },
                ..active
            }
        }

        fn dragging(&self) -> slider::Style {
            let active = self.active();

            slider::Style {
                handle: slider::Handle {
                    color: Color::from_rgb(0.85, 0.85, 0.85),
                    ..active.handle
                },
                ..active
            }
        }
    }

    pub struct ProgressBar;

    impl progress_bar::StyleSheet for ProgressBar {
        fn style(&self) -> progress_bar::Style {
            progress_bar::Style {
                background: SURFACE.into(),
                bar: ACTIVE.into(),
                border_radius: 10.0,
            }
        }
    }

    pub struct Checkbox;

    impl checkbox::StyleSheet for Checkbox {
        fn active(&self, is_checked: bool) -> checkbox::Style {
            checkbox::Style {
                background: if is_checked { ACTIVE } else { SURFACE }
                    .into(),
                checkmark_color: Color::WHITE,
                border_radius: 2.0,
                border_width: 1.0,
                border_color: ACTIVE,
            }
        }

        fn hovered(&self, is_checked: bool) -> checkbox::Style {
            checkbox::Style {
                background: Color {
                    a: 0.8,
                    ..if is_checked { ACTIVE } else { SURFACE }
                }
                .into(),
                ..self.active(is_checked)
            }
        }
    }

    pub struct Rule;

    impl rule::StyleSheet for Rule {
        fn style(&self) -> rule::Style {
            rule::Style {
                color: SURFACE,
                width: 2,
                radius: 1.0,
                fill_mode: rule::FillMode::Padded(15),
            }
        }
    }

    pub struct PickList;

    impl pick_list::StyleSheet for PickList {
        fn menu(&self) -> pick_list::Menu {
            pick_list::Menu {
                text_color: Color::WHITE.into(),
                background: Background::Color(SURFACE),
                border_width: 1.0,
                border_color: ACTIVE,
                selected_text_color: Color::WHITE.into(),
                selected_background: Background::Color(ACTIVE),
            }
        }

        fn active(&self) -> pick_list::Style {
            pick_list::Style {
                text_color: Color::WHITE.into(),
                background: Background::Color(SURFACE),
                border_width: 1.0,
                border_color: ACTIVE,
                border_radius: 3.0,
                icon_size: 0.7,
            }
        }

        fn hovered(&self) -> pick_list::Style {
            pick_list::Style {
                text_color: Color::WHITE.into(),
                background: Background::Color(SURFACE),
                border_width: 1.0,
                border_color: ACTIVE,
                border_radius: 3.0,
                icon_size: 0.7,
            }
        }
    }
}