use iced::widget::{button, container, text_input};
use iced::{application, Background, Border, Color, Shadow, Theme, Vector};

// --- Palette ---
// Dark
pub const VOID_BLACK: Color = Color::from_rgb(0.05, 0.05, 0.07); // #0D0D12
pub const SURFACE: Color = Color::from_rgb(0.1, 0.1, 0.13);     // #1A1A21
pub const ACCENT: Color = Color::from_rgb(0.23, 0.51, 0.96);    // #3B82F6 (Electric Blue)
pub const TEXT_PRIMARY: Color = Color::from_rgb(0.9, 0.9, 0.95);
pub const TEXT_SECONDARY: Color = Color::from_rgb(0.6, 0.6, 0.65);
pub const BORDER_COLOR: Color = Color::from_rgb(0.2, 0.2, 0.25);

// Light
pub const LIGHT_BACKGROUND: Color = Color::WHITE;
pub const LIGHT_SURFACE: Color = Color::from_rgb(0.95, 0.95, 0.97);
pub const LIGHT_TEXT_PRIMARY: Color = Color::BLACK;
pub const LIGHT_TEXT_SECONDARY: Color = Color::from_rgb(0.4, 0.4, 0.45);
pub const LIGHT_BORDER_COLOR: Color = Color::from_rgb(0.85, 0.85, 0.9);

// --- Styles ---

fn is_light(style: &Theme) -> bool {
    matches!(style, Theme::Light)
}

// 1. App Background
#[derive(Debug, Clone, Copy, Default)]
pub enum App {
    #[default]
    Default,
}

impl application::StyleSheet for App {
    type Style = Theme;

    fn appearance(&self, style: &Self::Style) -> application::Appearance {
        if is_light(style) {
            application::Appearance {
                background_color: LIGHT_BACKGROUND,
                text_color: LIGHT_TEXT_PRIMARY,
            }
        } else {
             application::Appearance {
                background_color: VOID_BLACK,
                text_color: TEXT_PRIMARY,
            }
        }
    }
}

impl From<App> for iced::theme::Application {
    fn from(style: App) -> Self {
        iced::theme::Application::Custom(Box::new(style))
    }
}

// 2. Text Input
#[derive(Debug, Clone, Copy, Default)]
pub enum TextInput {
    #[default]
    Default,
}

impl text_input::StyleSheet for TextInput {
    type Style = Theme;
    
    fn active(&self, style: &Self::Style) -> text_input::Appearance {
        if is_light(style) {
             text_input::Appearance {
                background: Background::Color(Color::WHITE),
                border: Border {
                    color: LIGHT_BORDER_COLOR,
                    width: 1.0,
                    radius: 8.0.into(),
                },
                icon_color: LIGHT_TEXT_SECONDARY,
            }
        } else {
            text_input::Appearance {
                background: Background::Color(Color::from_rgb(0.12, 0.12, 0.15)),
                border: Border {
                    color: BORDER_COLOR,
                    width: 1.0,
                    radius: 8.0.into(),
                },
                icon_color: TEXT_SECONDARY,
            }
        }
    }

    fn focused(&self, style: &Self::Style) -> text_input::Appearance {
        let active = self.active(style);
        text_input::Appearance {
            border: Border {
                color: ACCENT,
                ..active.border
            },
            ..active
        }
    }

    fn placeholder_color(&self, style: &Self::Style) -> Color {
        if is_light(style) {
             Color::from_rgb(0.6, 0.6, 0.65)
        } else {
             Color::from_rgb(0.4, 0.4, 0.45)
        }
    }

    fn value_color(&self, style: &Self::Style) -> Color {
        if is_light(style) {
            LIGHT_TEXT_PRIMARY
        } else {
            TEXT_PRIMARY
        }
    }
    
    fn disabled(&self, style: &Self::Style) -> text_input::Appearance {
        self.active(style) 
    }
    
    fn selection_color(&self, _style: &Self::Style) -> Color {
        Color::from_rgba(0.23, 0.51, 0.96, 0.2)
    }

    fn disabled_color(&self, style: &Self::Style) -> Color {
         if is_light(style) {
            LIGHT_TEXT_SECONDARY
        } else {
            TEXT_SECONDARY
        }
    }
}

impl From<TextInput> for iced::theme::TextInput {
    fn from(style: TextInput) -> Self {
        iced::theme::TextInput::Custom(Box::new(style))
    }
}

// 3. Containers & Sidebar
#[derive(Debug, Clone, Copy, Default)]
pub enum Container {
    #[default]
    Transparent,
    Background,
    Card,
    Sidebar,
}

impl container::StyleSheet for Container {
    type Style = Theme;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        let light = is_light(style);
        match self {
            Container::Transparent => container::Appearance::default(),
            Container::Background => container::Appearance {
                background: Some(Background::Color(if light { LIGHT_BACKGROUND } else { VOID_BLACK })),
                text_color: Some(if light { LIGHT_TEXT_PRIMARY } else { TEXT_PRIMARY }),
                ..Default::default()
            },
            Container::Card => container::Appearance {
                background: Some(Background::Color(if light { Color::WHITE } else { SURFACE })),
                border: Border {
                    color: if light { LIGHT_BORDER_COLOR } else { BORDER_COLOR },
                    width: 1.0,
                    radius: 12.0.into(),
                },
                shadow: Shadow {
                    color: if light { Color::from_rgba(0.0, 0.0, 0.0, 0.05) } else { Color::BLACK },
                    offset: Vector::new(0.0, 4.0),
                    blur_radius: 10.0,
                },
                text_color: Some(if light { LIGHT_TEXT_PRIMARY } else { TEXT_PRIMARY }),
            },
            Container::Sidebar => container::Appearance {
                background: Some(Background::Color(if light { LIGHT_SURFACE } else { SURFACE })),
                border: Border {
                    color: if light { LIGHT_BORDER_COLOR } else { BORDER_COLOR },
                    width: 0.0, // Or 1.0 right border
                    radius: 0.0.into(),
                },
                ..Default::default()
            },
        }
    }
}

impl From<Container> for iced::theme::Container {
    fn from(style: Container) -> Self {
        iced::theme::Container::Custom(Box::new(style))
    }
}


// 4. Buttons
#[derive(Debug, Clone, Copy, Default)]
pub enum Button {
    #[default]
    Primary,
    Secondary,
    Destructive,
    Menu, // For sidebar
}

impl button::StyleSheet for Button {
    type Style = Theme;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        let light = is_light(style);
        match self {
            Button::Primary => button::Appearance {
                background: Some(Background::Color(ACCENT)),
                text_color:  Color::WHITE,
                border: Border { radius: 8.0.into(), ..Default::default() },
                ..Default::default()
            },
            Button::Secondary => button::Appearance {
                background: Some(Background::Color(Color::TRANSPARENT)),
                text_color: if light { LIGHT_TEXT_PRIMARY } else { TEXT_PRIMARY },
                border: Border {
                    color: if light { LIGHT_BORDER_COLOR } else { BORDER_COLOR },
                    width: 1.0,
                    radius: 8.0.into(),
                },
                ..Default::default()
            },
            Button::Destructive => button::Appearance {
                background: Some(Background::Color(Color::from_rgb(0.8, 0.2, 0.2))),
                text_color: Color::WHITE,
                border: Border { radius: 8.0.into(), ..Default::default() },
                ..Default::default()
            },
            Button::Menu => button::Appearance {
                background: Some(Background::Color(Color::TRANSPARENT)),
                text_color: if light { LIGHT_TEXT_SECONDARY } else { TEXT_SECONDARY },
                ..Default::default()
            },
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);
        let light = is_light(style);
        match self {
            Button::Primary => button::Appearance {
                background: Some(Background::Color(Color::from_rgb(0.3, 0.6, 1.0))), // Lighter blue
                ..active
            },
            Button::Secondary => button::Appearance {
                border: Border { color: ACCENT, ..active.border },
                text_color: ACCENT,
                ..active
            },
            Button::Menu => button::Appearance {
                text_color: if light { LIGHT_TEXT_PRIMARY } else { TEXT_PRIMARY },
                background: Some(Background::Color(if light { Color::from_rgba(0.0, 0.0, 0.0, 0.05) } else { Color::from_rgba(1.0, 1.0, 1.0, 0.05) })),
                border: Border { radius: 6.0.into(), ..Default::default() }, 
                ..active
            },
            _ => active,
        }
    }
}

impl From<Button> for iced::theme::Button {
    fn from(style: Button) -> Self {
        iced::theme::Button::Custom(Box::new(style))
    }
}
