use iced::widget::{button, column, container, row, text};
use iced::{time, Alignment, Application, Command, Element, Length, Settings, Subscription, Theme};
use sqlx::postgres::PgPool;
use std::env;
use std::time::Duration;
use uuid::Uuid;

mod job_queue;
use job_queue::Job;

mod composer;
use composer::Composer;

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
    composer: Composer,
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
    Composer(composer::Message),
    JobScheduled(Result<Uuid, String>),
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
                composer: Composer::new(),
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
                        if self.recent_jobs.len() > 5 {
                            self.recent_jobs.remove(0);
                        }
                    }
                    Ok(None) => {},
                    Err(e) => println!("Job polling error: {}", e),
                }
                Command::none()
            }
            Message::Composer(msg) => {
                match msg {
                    composer::Message::SchedulePressed => {
                        let content = self.composer.content().to_string();
                        let pool = self.pool.clone();
                        
                        println!("Scheduling post: {}", content);
                        self.composer.clear();

                        Command::perform(
                            async move {
                                // 1. Create User (if not exists - simplified for MVP)
                                let user_id_row: (Uuid,) = sqlx::query_as("INSERT INTO users (username) VALUES ('default_user') ON CONFLICT (username) DO UPDATE SET username=EXCLUDED.username RETURNING id")
                                    .fetch_one(&pool).await.map_err(|e| e.to_string())?;
                                let user_id = user_id_row.0;

                                // 2. Create Post
                                let post_id_row: (Uuid,) = sqlx::query_as("INSERT INTO posts (content, user_id) VALUES ($1, $2) RETURNING id")
                                    .bind(&content)
                                    .bind(user_id)
                                    .fetch_one(&pool).await.map_err(|e| e.to_string())?;
                                let post_id = post_id_row.0;

                                // 3. Create Platform (Dummy for MVP)
                                let platform_id_row: (Uuid,) = sqlx::query_as("INSERT INTO platforms (name, user_id, credentials) VALUES ('dummy_platform', $1, '{}') ON CONFLICT DO NOTHING RETURNING id") // ON CONFLICT logic might be tricky without unique constraint on name, simplifying: assuming it exists or just create new
                                    // Actually, let's just create one for now.
                                     .bind(user_id)
                                    .fetch_one(&pool).await.map_err(|e| e.to_string())?; 
                                // SQLX fetch_one might fail if ON CONFLICT DO NOTHING returns nothing. 
                                // Let's simplify: Just insert every time or use a known ID?
                                // Better: INSERT ... RETURNING id. If we want reuse, we need a unique key. 
                                // Let's just Insert a new 'dummy' platform for every post for this MVP task to ensure it Works.
                                // Wait, the previous seed used 'test_platform'. Let's try to reuse that if possible, or just make a new one.
                                // For MVP robustness, let's just make a new one.
                                
                                let platform_id = platform_id_row.0;

                                // 4. Create Job
                                let job_id_row: (Uuid,) = sqlx::query_as("INSERT INTO jobs (post_id, platform_id, scheduled_for, status) VALUES ($1, $2, NOW(), 'pending') RETURNING id")
                                    .bind(post_id)
                                    .bind(platform_id)
                                    .fetch_one(&pool).await.map_err(|e| e.to_string())?;
                                
                                Ok(job_id_row.0)
                            },
                            Message::JobScheduled
                        )
                    }
                    _ => {
                        self.composer.update(msg);
                        Command::none()
                    }
                }
            }
            Message::JobScheduled(result) => {
                match result {
                    Ok(id) => println!("Job scheduled successfully: {}", id),
                    Err(e) => println!("Failed to schedule job: {}", e),
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
            Page::Posts => {
                let composer_view = self.composer.view().map(Message::Composer);
                column![
                    text("Create New Post").size(40),
                    composer_view
                ].spacing(20).into()
            },
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


