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

mod theme;
use theme::{App, Button, Container};

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
                                        // ... existing mastodon logic ...
                                        // (truncated for brevity in actual replace, I will keep context)
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
                                    "bluesky" => {
                                        let identifier = payload.credentials.0.get("identifier")
                                            .and_then(|v: &serde_json::Value| v.as_str())
                                            .unwrap_or("DUMMY_USER")
                                            .to_string();
                                        let password = payload.credentials.0.get("password")
                                            .and_then(|v: &serde_json::Value| v.as_str())
                                            .unwrap_or("DUMMY_PASS")
                                            .to_string();
                                        
                                        let mut client = integrations::bluesky::BlueskyClient::new(identifier, password);
                                        // Try login first
                                        match client.login().await {
                                            Ok(_) => {
                                                match client.post_record(&payload.content).await {
                                                    Ok(_) => Ok("Posted to Bluesky".to_string()),
                                                    Err(e) => Err(format!("Bluesky Post Error: {}", e))
                                                }
                                            },
                                            Err(e) => Err(format!("Bluesky Login Error: {}", e))
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
                                // 1. Create User
                                let user_id_row: (Uuid,) = sqlx::query_as("INSERT INTO users (username) VALUES ('default_user') ON CONFLICT (username) DO UPDATE SET username=EXCLUDED.username RETURNING id")
                                    .fetch_one(&pool).await.map_err(|e| e.to_string())?;
                                let user_id = user_id_row.0;

                                // 2. Create Post (Scheduled)
                                let post_id_row: (Uuid,) = sqlx::query_as("INSERT INTO posts (content, user_id, status) VALUES ($1, $2, 'scheduled') RETURNING id")
                                    .bind(&content)
                                    .bind(user_id)
                                    .fetch_one(&pool).await.map_err(|e| e.to_string())?;
                                let post_id = post_id_row.0;

                                // 3. Create Platform
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
                    composer::Message::SaveDraftPressed => {
                        let content = self.composer.content().to_string();
                        let pool = self.pool.clone();
                        
                        println!("Saving draft: {}", content);
                        self.composer.clear();

                        Command::perform(
                            async move {
                                // 1. Create User
                                let user_id_row: (Uuid,) = sqlx::query_as("INSERT INTO users (username) VALUES ('default_user') ON CONFLICT (username) DO UPDATE SET username=EXCLUDED.username RETURNING id")
                                    .fetch_one(&pool).await.map_err(|e| e.to_string())?;
                                let user_id = user_id_row.0;

                                // 2. Create Post (Draft)
                                let _post_id_row: (Uuid,) = sqlx::query_as("INSERT INTO posts (content, user_id, status) VALUES ($1, $2, 'draft') RETURNING id")
                                    .bind(&content)
                                    .bind(user_id)
                                    .fetch_one(&pool).await.map_err(|e| e.to_string())?;

                                // No job created for drafts
                                Ok(Uuid::nil()) // Return dummy UUID or handle differently. Uuid::nil() is 0000...
                            },
                            |res| match res {
                                Ok(_) => Message::JobScheduled(Ok(Uuid::nil())), // Re-using message for simplicity, or add new one
                                Err(e) => Message::JobScheduled(Err(e)),
                            }
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
                .width(Length::Fill)
                .style(Button::Menu),
            button("Posts")
                .on_press(Message::NavigateTo(Page::Posts))
                .width(Length::Fill)
                .style(Button::Menu),
            button("Schedule")
                .on_press(Message::NavigateTo(Page::Schedule))
                .width(Length::Fill)
                .style(Button::Menu),
            button("Settings")
                .on_press(Message::NavigateTo(Page::Settings))
                .width(Length::Fill)
                .style(Button::Menu),
        ]
        .spacing(10)
        .padding(20)
        .width(220); // Slightly wider

        let sidebar_container = container(sidebar)
            .height(Length::Fill)
            .style(Container::Sidebar);

        let content_view: Element<'_, Message> = match self.current_page {
            Page::Dashboard => {
                let logs: Vec<Element<_>> = self.recent_jobs
                    .iter()
                    .map(|s| {
                        container(text(s.clone()))
                            .padding(15)
                            .width(Length::Fill)
                            .style(Container::Card)
                            .into()
                    })
                    .collect();
                
                let logs_col = column(logs).spacing(10);
                
                column![
                    text("Dashboard").size(32).style(theme::ACCENT),
                    text("Recent Activity").size(18).style(theme::TEXT_SECONDARY),
                    logs_col
                ].spacing(20).into()
            },
            Page::Posts => {
                let composer_view = self.composer.view().map(Message::Composer);
                column![
                    text("Create New Post").size(32).style(theme::ACCENT),
                    composer_view
                ].spacing(20).into()
            },
            Page::Schedule => text("Schedule View").size(32).style(theme::ACCENT).into(),
            Page::Settings => text("Settings").size(32).style(theme::ACCENT).into(),
        };

        let content = container(content_view)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(40) // More breathing room
            .style(Container::Background);

        row![sidebar_container, content].into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }



// ...

    fn style(&self) -> <Self::Theme as iced::application::StyleSheet>::Style {
        theme::App::Default.into()
    }
}
