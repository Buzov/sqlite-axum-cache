use std::env;
use tracing::Level;
use tracing_appender::rolling;
use tracing_subscriber::{fmt, EnvFilter};
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub fn setup_logging() {
    let debug_level = env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());

    // File appender: daily logs to ./logs/app.log
    let file_appender = rolling::daily("./logs", "app.log")
        .with_max_level(Level::INFO);

    // Logging layer for file
    let file_layer = fmt::layer()
        .with_writer(file_appender)
        .with_ansi(false) // optional: disables ANSI colors in log file
        .with_level(true);

    // Logging layer for stdout
    let console_layer = fmt::layer()
        .with_ansi(true)
        .with_level(true); // shows the log level (INFO, DEBUG, etc.)

    // Set up tracing
    tracing_subscriber::registry()
        .with(EnvFilter::new(debug_level)) // enables filtering by RUST_LOG
        .with(file_layer)
        .with(console_layer)
        .init();
}
