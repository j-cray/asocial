use iced::widget::{button, column, container, row, text};
use iced::{time, Alignment, Application, Command, Element, Length, Settings, Subscription, Theme};
use sqlx::postgres::PgPool;
use std::env;
use std::time::Duration;

mod job_queue;
use job_queue::Job;

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
    recent_jobs: Vec<String>,
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
    Tick(time::Instant),
    JobPolled(Result<Option<Job>, String>),
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
                recent_jobs: Vec::new(),
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
            Message::Tick(_) => {
                // Poll for a job every tick
                Command::perform(
                    job_queue::poll_jobs(self.pool.clone()),
                    Message::JobPolled,
                )
            }
            Message::JobPolled(result) => {
                match result {
                    Ok(Some(job)) => {
                        let msg = format!("Processing Job: {}", job.id);
                        println!("{}", msg);
                        self.recent_jobs.push(msg);
                        // Keep only last 5 logs
                        if self.recent_jobs.len() > 5 {
                            self.recent_jobs.remove(0);
                        }
                    }
                    Ok(None) => {
                        // No jobs available, silent
                    }
                    Err(e) => {
                        println!("Job polling error: {}", e);
                    }
                }
                Command::none()
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        time::every(Duration::from_secs(5)).map(Message::Tick)
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

        let content_view: Element<'_, Message> = match self.current_page {
            Page::Dashboard => {
                let logs: Vec<Element<_>> = self.recent_jobs
                    .iter()
                    .map(|s| text(s.clone()).into())
                    .collect();
                
                let logs_col = column(logs).spacing(10);
                
                column![
                    text("Dashboard Content").size(40),
                    text("Recent Activity:").size(20),
                    logs_col
                ].spacing(20).into()
            },
            Page::Posts => text("Posts Management").size(40).into(),
            Page::Schedule => text("Schedule View").size(40).into(),
            Page::Settings => text("Settings").size(40).into(),
        };

        let content = container(content_view)
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

