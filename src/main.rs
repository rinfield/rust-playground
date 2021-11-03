use std::env::*;

mod eventlog;

fn main() {
    let args: Vec<String> = args().collect();
    let default_level = "WARN".to_string();
    let maybe_level: &str = args.get(1).unwrap_or(&default_level);
    let level = eventlog::LogLevel::try_from(maybe_level).unwrap_or(eventlog::LogLevel::WARN);
    println!("maybe_level: {}, level: {:?}", maybe_level, level);
    let logger = eventlog::Logger::new("fizzbuzz", eventlog::LogLevel::INFO);
    logger.error("errorです");
    logger.warn("warnです");
    logger.info("infoです");
    logger.debug("debugです");
}
