use log::*;
use std::env::*;
use std::str::FromStr;

mod eventlog;

fn main() {
    let args: Vec<String> = args().collect();

    let conf_level_filter = args
        .get(1)
        .and_then(|maybe_level| log::LevelFilter::from_str(maybe_level).ok());

    let eventlog: std::boxed::Box<dyn log::Log> =
        Box::new(eventlog::EventlogLogger::new("fizzbuzz"));

    fern::Dispatch::new()
        // Perform allocation-free log formatting
        .format(|out, message, record| out.finish(format_args!("[{}] {}", record.level(), message)))
        // Add blanket level filter -
        .level(conf_level_filter.unwrap_or(log::LevelFilter::Warn))
        // Output to stdout, files, and other Dispatch configurations
        .chain(std::io::stdout())
        .chain(eventlog)
        // Apply globally
        .apply()
        .unwrap();

    action1();
}

fn action1() {
    loop {
        error!("errorです");
        warn!("warnです");
        info!("infoです");
        debug!("debugです");
        trace!("traceです");
        std::thread::sleep(std::time::Duration::from_secs(3));
    }
}
