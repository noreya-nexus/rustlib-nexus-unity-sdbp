use std::env;
use log::{LevelFilter, set_max_level};
use rocket::log::LogLevel;

pub fn init_systemd_logger() {
    if env::var("INVOCATION_ID").is_ok()
        && (env::var("DISPLAY").is_err()
        && env::var("WAYLAND_DISPLAY").is_err()) // Do not log to systemd in graphical sessions
    {
        let log_level = parse_level();
        // Running as systemd service
        match systemd_journal_logger::JournalLog::default().install() {
            Ok(_) => {
                set_max_level(log_level.1);
                info!("systemd logger used");
            }
            Err(err) => {
                pretty_env_logger::init_custom_env("RUST_APP_LOG");
                panic!("systemd logger error: {:?}", err.to_string());
            }
        };
    } else {
        info!("pretty logger used");
        pretty_env_logger::init_custom_env("RUST_APP_LOG");
    }
}

pub fn parse_level() -> (LogLevel, LevelFilter) {
    let log_level = match env::var("RUST_APP_LOG") {
        Ok(val) => val,
        Err(_e) => "none".to_string(),
    };

    let log_level = match log_level.as_str() {
        "debug" => (LogLevel::Debug, LevelFilter::Debug),
        "info" => (LogLevel::Normal, LevelFilter::Info),
        "critical" => (LogLevel::Critical, LevelFilter::Error),
        _ => (LogLevel::Off, LevelFilter::Off),
    };

    return log_level;
}
