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
        .format(|out, message, _record| out.finish(format_args!("{}", message)))
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
    for i in 1.. {
        error!("{} errorです", i);
        warn!("{} warnです", i);
        info!("{} infoです", i);
        debug!("{} debugです", i);
        trace!("{} traceです", i);
    }
}
