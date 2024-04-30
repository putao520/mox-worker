use log4rs::append::file::FileAppender;
use log4rs::Config;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use log::{info, LevelFilter};

pub fn start_logging() {
    // 创建一个 Appender，将日志输出到文件
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} - {l} - {m}\n")))
        .append(false) // 设置为false以覆盖现有文件
        .build("log/test.log")
        .unwrap();

    // 创建一个 Appender，将日志输出到控制台
    let console = log4rs::append::console::ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} - {l} - {m}\n")))
        .build();

    // 构建日志配置
    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .appender(Appender::builder().build("console", Box::new(console)))
        .build(
            Root::builder()
                .appender("logfile")
                .appender("console")
                .build(LevelFilter::Info),
        )
        .unwrap();

    // 初始化日志系统
    log4rs::init_config(config).unwrap();

    // 输出日志
    info!("This is a test log message");
}