use clap::{App, Arg};
use mjcf_parser::MJCFModelDesc;
use nalgebra as na;
use nphysics3d::world::World;
use nphysics_testbed3d::Testbed;
use std::fs;

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
        .get_matches();

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
}
