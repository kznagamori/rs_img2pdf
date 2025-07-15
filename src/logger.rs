use crate::error::Result;
use fern::Dispatch;
use log::LevelFilter;

/// ログシステムを初期化する
///
/// # Arguments
/// * `level` - ログレベル文字列 ("trace", "debug", "info", "warn", "error")
///
/// # Returns
/// * `Result<()>` - 初期化の成功/失敗
pub fn init_logger(level: &str) -> Result<()> {
    let log_level = match level.to_lowercase().as_str() {
        "trace" => LevelFilter::Trace,
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        _ => LevelFilter::Info,
    };

    Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log_level)
        .chain(std::io::stdout())
        .apply()?;

    Ok(())
}
