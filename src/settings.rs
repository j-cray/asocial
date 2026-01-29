use iced::widget::{button, column, container, radio, row, text, text_input, vertical_space};
use iced::{Command, Element};
use crate::theme::{self, Button, TextInput};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ThemeMode {
    System,
    Light,
    #[default]
    Dark,
}

impl ThemeMode {
    pub const ALL: [ThemeMode; 3] = [ThemeMode::System, ThemeMode::Light, ThemeMode::Dark];
}

impl std::fmt::Display for ThemeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ThemeMode::System => "System",
                ThemeMode::Light => "Light",
                ThemeMode::Dark => "Dark",
            }
        )
    }
}

#[derive(Debug, Clone)]
pub struct Settings {
    pub theme_mode: ThemeMode,
    mastodon_url: String,
    mastodon_token: String,
    bluesky_identifier: String,
    bluesky_password: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    ThemeChanged(ThemeMode),
    MastodonUrlChanged(String),
    MastodonTokenChanged(String),
    BlueskyIdentifierChanged(String),
    BlueskyPasswordChanged(String),
    SaveAccountsPressed,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            theme_mode: ThemeMode::Dark,
            mastodon_url: String::new(),
            mastodon_token: String::new(),
            bluesky_identifier: String::new(),
            bluesky_password: String::new(),
        }
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ThemeChanged(mode) => {
                self.theme_mode = mode;
                Command::none()
            }
            Message::MastodonUrlChanged(val) => {
                self.mastodon_url = val;
                Command::none()
            }
            Message::MastodonTokenChanged(val) => {
                self.mastodon_token = val;
                Command::none()
            }
            Message::BlueskyIdentifierChanged(val) => {
                self.bluesky_identifier = val;
                Command::none()
            }
            Message::BlueskyPasswordChanged(val) => {
                self.bluesky_password = val;
                Command::none()
            }
            Message::SaveAccountsPressed => {
                 // Main app will handle the actual DB save
                 Command::none()
            }
        }
    }

    pub fn mastodon_creds(&self) -> (String, String) {
        (self.mastodon_url.clone(), self.mastodon_token.clone())
    }

    pub fn bluesky_creds(&self) -> (String, String) {
        (self.bluesky_identifier.clone(), self.bluesky_password.clone())
    }

    pub fn view(&self) -> Element<'_, Message> {
        let theme_section = column![
            text("Appearance").size(24).style(theme::ACCENT),
            row(
                ThemeMode::ALL.iter().map(|mode| {
                    radio(
                        format!("{}", mode),
                        *mode,
                        Some(self.theme_mode),
                        Message::ThemeChanged,
                    ).into()
                }).collect::<Vec<_>>()
            ).spacing(20)
        ].spacing(10);

        let mastodon_section = column![
            text("Mastodon").size(24).style(theme::ACCENT),
            text_input("Instance URL (e.g. https://mastodon.social)", &self.mastodon_url)
                .on_input(Message::MastodonUrlChanged)
                .padding(10)
                .style(TextInput::Default),
            text_input("Access Token", &self.mastodon_token)
                .on_input(Message::MastodonTokenChanged)
                .padding(10)
                .secure(true)
                .style(TextInput::Default),
        ].spacing(10);

        let bluesky_section = column![
             text("Bluesky").size(24).style(theme::ACCENT),
             text_input("Identifier (e.g. user.bsky.social)", &self.bluesky_identifier)
                .on_input(Message::BlueskyIdentifierChanged)
                .padding(10)
                .style(TextInput::Default),
            text_input("App Password", &self.bluesky_password)
                .on_input(Message::BlueskyPasswordChanged)
                .padding(10)
                .secure(true) 
                .style(TextInput::Default),
        ].spacing(10);

        let content = column![
            theme_section,
            vertical_space().height(20),
            mastodon_section,
            vertical_space().height(10),
            bluesky_section,
            vertical_space().height(20),
            button("Save Accounts")
                .on_press(Message::SaveAccountsPressed)
                .padding(10)
                .style(Button::Primary)
        ]
        .spacing(20);

        container(content).into()
    }
}
