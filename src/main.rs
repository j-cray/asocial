use iced::widget::{button, column, container, row, text, Container};
use iced::{Alignment, Element, Length, Sandbox, Settings, Theme};

pub fn main() -> iced::Result {
    AsocialApp::run(Settings::default())
}

struct AsocialApp {
    current_page: Page,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Page {
    Dashboard,
    Posts,
    Schedule,
    Settings,
}

#[derive(Debug, Clone)]
enum Message {
    NavigateTo(Page),
}

impl Sandbox for AsocialApp {
    type Message = Message;

    fn new() -> Self {
        Self {
            current_page: Page::Dashboard,
        }
    }

    fn title(&self) -> String {
        String::from("Asocial")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::NavigateTo(page) => {
                self.current_page = page;
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let sidebar = column![
            button("Dashboard")
                .on_press(Message::NavigateTo(Page::Dashboard))
                .width(Length::Fill),
            button("Posts")
                .on_press(Message::NavigateTo(Page::Posts))
                .width(Length::Fill),
            button("Schedule")
                .on_press(Message::NavigateTo(Page::Schedule))
                .width(Length::Fill),
            button("Settings")
                .on_press(Message::NavigateTo(Page::Settings))
                .width(Length::Fill),
        ]
        .spacing(20)
        .padding(20)
        .width(200)
        .align_items(Alignment::Center);

        let content = container(match self.current_page {
            Page::Dashboard => text("Dashboard Content").size(40),
            Page::Posts => text("Posts Management").size(40),
            Page::Schedule => text("Schedule View").size(40),
            Page::Settings => text("Settings").size(40),
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y();

        row![sidebar, content].into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
