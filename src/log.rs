use crossbeam::atomic::AtomicCell;
use slog;
use slog::Drain;
use slog_stdlog;

lazy_static! {
    pub static ref LOG: AtomicCell<slog::Logger> =
        AtomicCell::new(slog::Logger::root(slog_stdlog::StdLog.fuse(), o!()));
}

pub fn set_root_logger(logger: slog::Logger) {
    LOG.store(slog::Logger::root(logger, o!()));
}
