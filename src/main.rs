mod editor;
mod terminal;
mod document;

use editor::Editor;
use log::LevelFilter;
use log4rs::{Config, config::{Appender, Logger, Root}, encode::pattern::PatternEncoder};
use log4rs::append::file::FileAppender;

fn main() {
    let trace_logs = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} - {m}{n}")))
        .build("hecto_logs.log")
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("trace_logs", Box::new(trace_logs)))
        .logger(Logger::builder()
            .appender("trace_logs")
            .additive(false)
            .build("trace", LevelFilter::Info))
        .build(Root::builder().appender("trace_logs").build(LevelFilter::Info))
        .unwrap();

    let _handle = log4rs::init_config(config).unwrap();

    Editor::default().run();
}
