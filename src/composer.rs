use iced::{Command, Element};
use iced::widget::{checkbox, column, text, row, image};
use std::path::PathBuf;
use std::collections::HashMap;
use crate::theme::{self, Button, TextInput};

#[derive(Debug, Clone)]
pub struct Composer {
    content: String,
    selected_images: Vec<PathBuf>,
    platforms: HashMap<String, bool>,
}

#[derive(Debug, Clone)]
pub enum Message {
    ContentChanged(String),
    SchedulePressed,
    SaveDraftPressed,
    AttachImagePressed,
    ImageSelected(Option<PathBuf>),
    PlatformToggled(String, bool),
}

impl Composer {
    pub fn new() -> Self {
        let mut platforms = HashMap::new();
        platforms.insert("mastodon".to_string(), true);
        platforms.insert("bluesky".to_string(), true);

        Self {
            content: String::new(),
            selected_images: Vec::new(),
            platforms,
        }
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ContentChanged(content) => {
                self.content = content;
                Command::none()
            }
            Message::AttachImagePressed => {
                Command::perform(
                    async {
                        let handle = rfd::AsyncFileDialog::new()
                            .add_filter("Images", &["png", "jpg", "jpeg", "gif", "webp"])
                            .pick_file()
                            .await;
                        
                        handle.map(|h| h.path().to_path_buf())
                    },
                    Message::ImageSelected
                )
            }
            Message::ImageSelected(path) => {
                if let Some(p) = path {
                    self.selected_images.push(p);
                }
                Command::none()
            }
            Message::PlatformToggled(platform, is_active) => {
                self.platforms.insert(platform, is_active);
                Command::none()
            }
            Message::SchedulePressed | Message::SaveDraftPressed => {
                // Main app handles the actual DB insertion for now, but we might verify validity here
                Command::none()
            }
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn media_paths(&self) -> Vec<String> {
        self.selected_images.iter()
            .map(|p| p.to_string_lossy().to_string())
            .collect()
    }

    pub fn enabled_platforms(&self) -> Vec<String> {
        self.platforms.iter()
            .filter(|(_, &active)| active)
            .map(|(k, _)| k.clone())
            .collect()
    }
    
    pub fn clear(&mut self) {
        self.content.clear();
        self.selected_images.clear();
    }

    pub fn view(&self) -> Element<'_, Message> {
        // 1. Text Input
        let input = iced::widget::text_input("What's on your mind?", &self.content)
            .on_input(Message::ContentChanged)
            .padding(15)
            .size(20)
            .style(TextInput::Default);

        // 2. Image Preview
        let images_view: Element<'_, Message> = if !self.selected_images.is_empty() {
             let previews: Vec<Element<_>> = self.selected_images.iter()
                .map(|path| {
                    text(path.file_name().unwrap_or_default().to_string_lossy()).into()
                })
                .collect();
            row(previews).spacing(10).into()
        } else {
             text("").into()
        };

        // 3. Controls (Attach + Platforms)
        let controls = row![
             iced::widget::button("Attach Image")
                .on_press(Message::AttachImagePressed)
                .padding(10)
                .style(Button::Secondary),
             
             // Checkboxes
             row![
                 checkbox("Mastodon", self.platforms.get("mastodon").cloned().unwrap_or(false))
                    .on_toggle(|b| Message::PlatformToggled("mastodon".to_string(), b)),
                 checkbox("Bluesky", self.platforms.get("bluesky").cloned().unwrap_or(false))
                     .on_toggle(|b| Message::PlatformToggled("bluesky".to_string(), b)),
             ].spacing(15)
        ].spacing(20).align_items(iced::Alignment::Center);

        // 4. Main Buttons
        let main_actions = iced::widget::row![
            iced::widget::button("Save Draft")
                .on_press(Message::SaveDraftPressed)
                .padding(12)
                .style(Button::Secondary),
            iced::widget::button("Schedule Post")
                .on_press(Message::SchedulePressed)
                .padding(12)
                .style(Button::Primary)
        ].spacing(15);

        column![
            input,
            images_view,
            controls,
            main_actions
        ]
        .spacing(20)
        .into()
    }
}
