use axum::{
    extract::{Form, State},
    response::{IntoResponse, Redirect},
    routing::{get, post},
    Router,
};
use redis::{AsyncCommands, Client};
use serde::Deserialize;
use std::env;
use std::sync::Arc;
use tera::Tera;

struct AppState {
    redis_client: Client,
    tera: Tera,
}

#[derive(Deserialize)]
struct GuestbookForm {
    message: String,
}

mod ax_templates {
    use axum::{
        http::StatusCode,
        response::{Html, IntoResponse, Response},
    };
    use tera::{Context, Tera};

    pub struct TeraHtml(pub String, pub Context);

    impl TeraHtml {
        pub fn render(self, tera: &Tera) -> Response {
            match tera.render(&self.0, &self.1) {
                Ok(html) => Html(html).into_response(),
                Err(err) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Template error: {}", err),
                )
                    .into_response(),
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Configuration
    let redis_host = env::var("REDISHOST").unwrap_or_else(|_| "redis-leader".to_string());
    let redis_url = format!("redis://{}/", redis_host);
    
    let redis_client = Client::open(redis_url).expect("Invalid Redis URL");
    
    let mut tera = Tera::default();
    tera.add_template_files(vec![("templates/index.html", Some("index.html"))])
        .expect("Failed to load templates");

    let state = Arc::new(AppState {
        redis_client,
        tera,
    });

    let app = Router::new()
        .route("/", get(show_guestbook))
        .route("/guestbook", post(add_to_guestbook))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn show_guestbook(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let mut conn = state
        .redis_client
        .get_multiplexed_tokio_connection()
        .await
        .expect("Failed to connect to Redis");

    // The original PHP logic used a comma-separated string in a single key 'messages'
    // but a Redis LIST is much more idiomatic for this.
    // However, to stay true to the requested "append to list" logic:
    let messages: Vec<String> = conn.lrange("messages", 0, -1).await.unwrap_or_default();

    let mut context = tera::Context::new();
    context.insert("messages", &messages);

    ax_templates::TeraHtml("index.html".to_string(), context).render(&state.tera)
}

async fn add_to_guestbook(
    State(state): State<Arc<AppState>>,
    Form(form): Form<GuestbookForm>,
) -> impl IntoResponse {
    let mut conn = state
        .redis_client
        .get_multiplexed_tokio_connection()
        .await
        .expect("Failed to connect to Redis");

    if !form.message.trim().is_empty() {
        let _: () = conn
            .rpush("messages", form.message)
            .await
            .expect("Failed to write to Redis");
    }

    Redirect::to("/")
}
