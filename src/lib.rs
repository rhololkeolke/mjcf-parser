pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}
mod log;
mod mjcf_model;

pub use log::{drop_root_logger, set_root_logger};
pub use mjcf_model::MJCFModel;
