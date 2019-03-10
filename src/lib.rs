#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate slog;

use na::Real;
use nalgebra as na;
use ncollide3d::shape::ShapeHandle;
use nphysics3d::material::MaterialHandle;
use nphysics3d::object::ColliderDesc;
use roxmltree;
use std::collections::HashMap;

pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}
pub mod log;

pub struct MJCFModel<N: Real> {
    model_name: String,
    shapes: HashMap<String, ShapeHandle<N>>,
    colliders: HashMap<String, ColliderDesc<N>>,
    materials: HashMap<String, MaterialHandle<N>>,
}

impl<N: Real> MJCFModel<N> {
    // TODO(dschwab): proper return type and error type
    pub fn parse_xml_string(text: &str) -> Result<MJCFModel<N>, String> {
        let mut mjcf_model = MJCFModel {
            model_name: String::from("MuJoCo Model"),
            shapes: HashMap::new(),
            colliders: HashMap::new(),
            materials: HashMap::new(),
        };

        let doc = match roxmltree::Document::parse(text) {
            Ok(doc) => doc,
            Err(error) => return Err(format!("{:?}", error)),
        };

        let root = doc.root_element();

        // TODO(dschwab): change this to a proper error
        assert!(root.has_tag_name("mujoco"));
        if let Some(model_name) = root.attribute("model") {
            mjcf_model.model_name = model_name.to_string();
        }

        for child in root.children() {
            match child.tag_name().name() {
                "worldbody" => mjcf_model.parse_worldbody(&child),
                _ => {}
            };
        }

        Ok(mjcf_model)
    }

    fn parse_worldbody(&mut self, worldbody_node: &roxmltree::Node) {}
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
