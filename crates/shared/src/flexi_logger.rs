//! 初始化 flexi_logger 日志.

use anyhow::Result;
use flexi_logger::{
    Age, Cleanup, Criterion, DeferredNow, Duplicate, FileSpec, Logger, LoggerHandle, Naming,
    WriteMode,
};
use log::Record;
use std::{
    time::Duration,
    {env, thread},
};

/// 初始化 flexi_logger
pub fn init_flexi_logger() -> Result<LoggerHandle> {
    let log_name = env::var("LOG_NAME")?;
    let log_level = env::var("LOG_LEVEL")?;

    let log_directory: String;
    #[cfg(debug_assertions)]
    {
        log_directory = format!("log/{}", log_name);
    }
    #[cfg(not(debug_assertions))]
    {
        log_directory = format!("/var/log/{}", log_level);
    }

    let file_spec = FileSpec::default()
        .directory(log_directory)
        .basename(log_name)
        .suppress_timestamp();

    let mut logger = Logger::try_with_str(log_level)?;
    logger = logger.log_to_file(file_spec);
    logger = logger.format(log_format);

    #[cfg(debug_assertions)]
    {
        logger = logger.duplicate_to_stdout(Duplicate::All);
    }
    #[cfg(not(debug_assertions))]
    {
        logger = logger.duplicate_to_stdout(Duplicate::Error);
    }

    logger = logger.write_mode(WriteMode::BufferAndFlushWith(64000, Duration::from_secs(5)));

    // 1GB = 1000000000 bytes
    // 10MB = 10000000 bytes
    // 50MB = 50000000 bytes
    // 100MB = 100000000 bytes
    logger = logger
        .rotate(
            Criterion::AgeOrSize(Age::Day, 10000000),
            Naming::Timestamps,
            Cleanup::KeepLogFiles(30),
        )
        .append();
    Ok(logger.start()?)
}

/// 自定义日志格式
fn log_format(
    w: &mut dyn std::io::Write,
    now: &mut DeferredNow,
    record: &Record,
) -> Result<(), std::io::Error> {
    write!(
        w,
        "[{}] T[{}] {} [{}:{}] ",
        now.format("%Y-%m-%d %H:%M:%S%.6f"),
        thread::current().name().unwrap_or("<unnamed>"),
        record.level(),
        record.module_path().unwrap_or("<unnamed>"),
        record.line().unwrap_or(0),
    )?;

    // #[cfg(feature = "kv")]
    // write_key_value_pairs(w, record)?;

    write!(w, "{}", &record.args())
}
