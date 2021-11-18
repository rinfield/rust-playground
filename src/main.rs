use log::*;
use std::env;
use std::str::FromStr;

mod eventlog;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let conf_level_filter = args
        .get(1)
        .and_then(|maybe_level| log::LevelFilter::from_str(maybe_level).ok());

    let conf_iteration = args
        .get(2)
        .and_then(|maybe_usize| usize::from_str(maybe_usize).ok());

    let eventlog: Box<dyn log::Log> = Box::new(eventlog::EventlogLogger::new("fizzbuzz"));
    fern::Dispatch::new()
        .level(conf_level_filter.unwrap_or(log::LevelFilter::Warn))
        .chain(eventlog)
        .apply()
        .unwrap();

    action1(conf_iteration.unwrap_or(1));
}

fn action1(iteration: usize) {
    for i in 1..=iteration {
        error!("{} errorです", i);
        warn!("{} warnです", i);
        info!("{} infoです", i);
        debug!("{} debugです", i);
        trace!("{} traceです", i);
    }
}
