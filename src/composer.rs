use iced::widget::{button, column, text_input};
use iced::Element;

#[derive(Debug, Clone)]
pub struct Composer {
    content: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    ContentChanged(String),
    SchedulePressed,
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
            Message::SchedulePressed => {
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
            text_input("What's on your mind?", &self.content)
                .on_input(Message::ContentChanged)
                .padding(10)
                .size(20),
            button("Schedule Post")
                .on_press(Message::SchedulePressed)
                .padding(10)
        ]
        .spacing(20)
        .into()
    }
}
