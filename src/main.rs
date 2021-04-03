mod document;
mod editor;
mod terminal;

use editor::Editor;
use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::{
    config::{Appender, Logger, Root},
    encode::pattern::PatternEncoder,
    Config,
};

fn main() {
    let trace_logs = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} - {m}{n}")))
        .build("hecto_logs.log")
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("trace_logs", Box::new(trace_logs)))
        .logger(
            Logger::builder()
                .appender("trace_logs")
                .additive(false)
                .build("trace", LevelFilter::Info),
        )
        .build(
            Root::builder()
                .appender("trace_logs")
                .build(LevelFilter::Info),
        )
        .unwrap();

    let _handle = log4rs::init_config(config).unwrap();

    Editor::default().run();
}
