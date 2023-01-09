use log::LevelFilter;
use log4rs::{
    self,
    append::{
        console::{ConsoleAppender, Target},
        file::FileAppender,
    },
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
    Config,
};

// TODO: the panics are only logged to the console not the file
//TODO: use tui logger or env logger instead
// or stay with the new log4rs config?
pub struct Logger {}
impl Logger {
    pub fn init() {
        log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
        // let level = log::LevelFilter::Info;
        // let file_path = "log/app.log";

        // // Build a stderr logger.
        // let stderr = ConsoleAppender::builder().target(Target::Stderr).build();

        // // Logging to log file.
        // let logfile = FileAppender::builder()
        //     // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
        //     .encoder(Box::new(PatternEncoder::new(
        //         "{d(%+)(utc)} [{f}:{L}] {h({l})}: {m}{n}",
        //     )))
        //     .build(file_path)
        //     .unwrap();

        // // Log Trace level output to file where trace is the default level
        // // and the programmatically specified level to stderr.
        // let config = Config::builder()
        //     .appender(Appender::builder().build("logfile", Box::new(logfile)))
        //     .appender(
        //         Appender::builder()
        //             .filter(Box::new(ThresholdFilter::new(level)))
        //             .build("stderr", Box::new(stderr)),
        //     )
        //     .build(
        //         Root::builder()
        //             .appender("logfile")
        //             .appender("stderr")
        //             .build(LevelFilter::Debug),
        //     )
        //     .unwrap();

        // // Use this to change log levels at runtime.
        // // This means you can change the default log level to trace
        // // if you are trying to debug an issue and need more logs on then turn it off
        // // once you are done.
        // let _handle = log4rs::init_config(config);
    }
}
