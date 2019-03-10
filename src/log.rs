use crate::built_info;
use crossbeam::atomic::AtomicCell;
use slog;
use slog::Drain;
use slog_stdlog;

lazy_static! {
    pub static ref LOG: AtomicCell<slog::Logger> = AtomicCell::new(create_root_logger(None));
}

fn create_root_logger<L: Into<Option<slog::Logger>>>(logger: L) -> slog::Logger {
    let values = o!("mjcf-parser/version" => built_info::PKG_VERSION,
                    "mjcf-parser/commit" => format!("{:?}", built_info::GIT_VERSION),
                    "mjcf-parser/profile" => built_info::PROFILE);
    match logger.into() {
        Some(logger) => slog::Logger::root(logger, values),
        None => slog::Logger::root(slog_stdlog::StdLog.fuse(), values),
    }
}

pub fn set_root_logger<L: Into<slog::Logger>>(logger: L) {
    LOG.store(create_root_logger(Some(logger.into())));
}
