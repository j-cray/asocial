use iced::widget::{button, column, container, row, text};
use iced::{Alignment, Application, Command, Element, Length, Settings, Theme};
use sqlx::postgres::PgPool;
use std::env;

#[tokio::main]
pub async fn main() -> iced::Result {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    println!("Connecting to database: {}", database_url);
    
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    println!("Database connection established.");

    AsocialApp::run(Settings::with_flags(pool))
}

struct AsocialApp {
    pool: PgPool,
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

impl Application for AsocialApp {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = PgPool;

    fn new(pool: PgPool) -> (Self, Command<Message>) {
        (
            Self {
                pool,
                current_page: Page::Dashboard,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Asocial")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::NavigateTo(page) => {
                self.current_page = page;
                Command::none()
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
