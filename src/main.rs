use log::*;
// use std::env::*;

mod eventlog;

fn main() {
    // let args: Vec<String> = args().collect();
    // let default_level = "WARN".to_string();
    // let maybe_level: &str = args.get(1).unwrap_or(&default_level);
    // let level = eventlog::LogLevel::try_from(maybe_level).unwrap_or(eventlog::LogLevel::WARN);
    // println!("maybe_level: {}, level: {:?}", maybe_level, level);
    let eventlog: std::boxed::Box<dyn log::Log> =
        Box::new(eventlog::EventlogLogger::new("fizzbuzz"));

    fern::Dispatch::new()
        // Perform allocation-free log formatting
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{}][{}] {}",
                chrono::Local::now().to_rfc3339(),
                record.target(),
                record.level(),
                message
            ))
        })
        // Add blanket level filter -
        .level(log::LevelFilter::Warn)
        // Output to stdout, files, and other Dispatch configurations
        .chain(std::io::stdout())
        .chain(eventlog)
        // Apply globally
        .apply()
        .unwrap();

    error!("errorです");
    warn!("warnです");
    info!("infoです");
    debug!("debugです");
    trace!("traceです");
}
