#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate slog;

pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}
mod log;
mod mjcf_model;

pub use log::set_root_logger;
pub use mjcf_model::MJCFModel;
