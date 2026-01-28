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

mod integrations;
use integrations::mastodon::MastodonClient;

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
    Tick(()),
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
                        self.recent_jobs.push(msg.clone());
                        if self.recent_jobs.len() > 5 {
                            self.recent_jobs.remove(0);
                        }

                        // Dispatch Job
                        let pool = self.pool.clone();
                        let job_id = job.id;
                        
                        Command::perform(
                            async move {
                                let payload = job_queue::fetch_job_details(pool.clone(), job_id).await?;
                                println!("Payload fetched for platform: {}", payload.platform_name);

                                match payload.platform_name.as_str() {
                                    "mastodon" | "test_platform" | "dummy_platform" => {
                                        // TODO: Extract real creds from JSON. For MVP stubbing if empty.
                                        let base_url = payload.api_url.unwrap_or_else(|| "https://mastodon.social".to_string());
                                        let token = payload.credentials.0.get("access_token")
                                            .and_then(|v: &serde_json::Value| v.as_str())
                                            .unwrap_or("DUMMY_TOKEN")
                                            .to_string();

                                        let client = MastodonClient::new(base_url, token);
                                        // In a real scenario, we'd check if token is dummy and fail or log.
                                        // For MVP, we try to post. If it fails (401), we log it.
                                        match client.post_status(&payload.content).await {
                                            Ok(_) => Ok("Posted to Mastodon".to_string()),
                                            Err(e) => Err(format!("Mastodon Error: {}", e))
                                        }
                                    },
                                    _ => Err(format!("Unknown platform: {}", payload.platform_name))
                                }
                            },
                            |res| {
                                match res {
                                    Ok(s) => println!("Job Success: {}", s),
                                    Err(e) => println!("Job Failed: {}", e),
                                }
                                Message::Tick(())
                            } 
                        )
                    }
                    Ok(None) => Command::none(),
                    Err(e) => {
                        println!("Job polling error: {}", e);
                        Command::none()
                    }
                }
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
                                let platform_id_row: (Uuid,) = sqlx::query_as("INSERT INTO platforms (name, user_id, credentials) VALUES ('dummy_platform', $1, '{}') ON CONFLICT DO NOTHING RETURNING id") 
                                     .bind(user_id)
                                    .fetch_one(&pool).await.map_err(|e| e.to_string())?; 
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
        time::every(Duration::from_secs(5)).map(|_| Message::Tick(()))
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
