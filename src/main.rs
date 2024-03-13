use errors::Result;
use tracing::dispatcher::set_global_default;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{fmt, layer::SubscriberExt, EnvFilter, Layer, Registry};

mod api;
mod errors;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let _guard = setup_logging();

    let router = api::app();

    listen(router).await?;

    Ok(())
}

async fn listen(router: axum::Router) -> Result<()> {
    let port = std::env::var("PORT").unwrap_or("3000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(addr).await?;

    tracing::info!("listening on {}", listener.local_addr()?);

    axum::serve(listener, router).await?;
    Ok(())
}

const DEFAULT_LOG_FILTER: &str = "api_template=info,tower_http=info";
const DEFAULT_LOG_PATH: &str = "/logs";

fn setup_logging() -> impl Drop {
    let log_path = std::env::var("LOG_PATH").unwrap_or(DEFAULT_LOG_PATH.to_string());
    let file_appender = RollingFileAppender::new(Rotation::DAILY, log_path, "app.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    let file_layer = fmt::layer().with_writer(non_blocking).json().with_filter(
        EnvFilter::try_from_default_env().unwrap_or_else(|_| DEFAULT_LOG_FILTER.into()),
    );

    let console_layer = fmt::layer().with_writer(std::io::stdout).with_filter(
        EnvFilter::try_from_default_env().unwrap_or_else(|_| DEFAULT_LOG_FILTER.into()),
    );

    let subscriber = Registry::default().with(file_layer).with(console_layer);

    set_global_default(subscriber.into()).expect("Failed to set global log subsriber");

    guard
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
