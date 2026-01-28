use iced::Element;
use iced::widget::column;
use crate::theme::{self, Button, TextInput};

#[derive(Debug, Clone)]
pub struct Composer {
    content: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    ContentChanged(String),
    SchedulePressed,
    SaveDraftPressed,
}

impl Composer {
    pub fn new() -> Self {
        Self {
            content: String::new(),
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::ContentChanged(content) => {
                self.content = content;
            }
            Message::SchedulePressed | Message::SaveDraftPressed => {
                // Main app handles the actual DB insertion for now
            }
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
    
    pub fn clear(&mut self) {
        self.content.clear();
    }

    pub fn view(&self) -> Element<'_, Message> {
        column![
            iced::widget::text_input("What's on your mind?", &self.content)
                .on_input(Message::ContentChanged)
                .padding(15)
                .size(20)
                .style(TextInput::Default),
            iced::widget::row![
                iced::widget::button("Save Draft")
                    .on_press(Message::SaveDraftPressed)
                    .padding(12)
                    .style(Button::Secondary),
                iced::widget::button("Schedule Post")
                    .on_press(Message::SchedulePressed)
                    .padding(12)
                    .style(Button::Primary)
            ].spacing(15)
        ]
        .spacing(20)
        .into()
    }
}
