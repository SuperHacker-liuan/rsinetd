use crate::CONFIG;
use simplelog::CombinedLogger;
use simplelog::LevelFilter;
use simplelog::SharedLogger;
use simplelog::TermLogger;
use simplelog::TerminalMode;

pub fn init_logger() {
    let mut logger: Vec<Box<dyn SharedLogger>> = vec![];
    if !CONFIG.daemon {
        let term = TermLogger::new(LevelFilter::Debug, config(), TerminalMode::Mixed);
        logger.push(term);
    }
    CombinedLogger::init(logger).expect("Failed to init logger");
}

fn config() -> simplelog::Config {
    simplelog::ConfigBuilder::new()
        .set_time_format_str("%F %T")
        .build()
}
