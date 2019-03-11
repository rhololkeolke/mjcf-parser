use clap::{App, Arg};
use mjcf_parser::MJCFModelDesc;
use nalgebra as na;
use nphysics3d::world::World;
use nphysics_testbed3d::Testbed;
use slog;
use slog::o;
use slog::Drain;
use slog_async;
use slog_term;
use std::fs;

fn parse_level(level: &str) -> slog::Level {
    match level.trim().to_lowercase().as_str() {
        "trace" => slog::Level::Trace,
        "debug" => slog::Level::Debug,
        "info" => slog::Level::Info,
        "warn" => slog::Level::Warning,
        "error" => slog::Level::Error,
        "critical" => slog::Level::Critical,
        _ => panic!(
            "Unknown log level {}. Must be one of [trace, debug, info, warn, error, critical]",
            level
        ),
    }
}

fn make_logger(level: slog::Level, model_file: String) -> slog::Logger {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = drain.filter_level(level).fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    slog::Logger::root(
        drain,
        o!("model_file" => model_file,
            "place" =>
           slog::FnValue(move |info| {
                       format!("{}:{} {}",
                               info.file(),
                               info.line(),
                               info.module(),
                               )
        })),
    )
}

fn main() {
    let matches = App::new("Model Explorer")
        .version("0.1")
        .author("Devin Schwab <dschwab@andrew.cmu.edu>")
        .about("Interactively simulate an MJCF model using nphysics")
        .arg(
            Arg::with_name("MODEL_FILE")
                .help("Model file to load and simulate")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("log_level")
                .short("l")
                .long("log-level")
                .value_name("LEVEL")
                .help("Set the logging level")
                .takes_value(true),
        )
        .get_matches();

    let logger = make_logger(
        parse_level(matches.value_of("log_level").unwrap_or("info")),
        matches.value_of("MODEL_FILE").unwrap().to_string(),
    );
    mjcf_parser::set_root_logger(logger.clone());

    let model_xml = fs::read_to_string(matches.value_of("MODEL_FILE").unwrap())
        .expect("Failed to read model file");

    let mut model_desc =
        MJCFModelDesc::parse_xml_string(&model_xml).expect("Failed to parse model file xml");

    // TODO(dschwab): get the gravity from the model desc
    let mut world = World::new();
    world.set_gravity(na::Vector3::z() * -9.81);

    // build the model desc
    model_desc.build(&mut world);

    // create the testbed
    let mut testbed = Testbed::new(world);
    testbed.look_at(
        na::Point3::new(0.0, 0.0, 2.0),
        na::Point3::new(0.0, 0.0, 1.0),
    );
    testbed.run();

    mjcf_parser::drop_root_logger();
}
