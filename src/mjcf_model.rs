use crate::log;
use na::Real;
use nalgebra as na;
use ncollide3d::shape::ShapeHandle;
use nphysics3d::material::MaterialHandle;
use nphysics3d::object::ColliderDesc;
use roxmltree;
use std::collections::HashMap;

pub struct MJCFModel<N: Real> {
    model_name: String,
    shapes: HashMap<String, ShapeHandle<N>>,
    colliders: HashMap<String, ColliderDesc<N>>,
    materials: HashMap<String, MaterialHandle<N>>,
}

impl<N: Real> MJCFModel<N> {
    // TODO(dschwab): proper return type and error type
    pub fn parse_xml_string(text: &str) -> Result<MJCFModel<N>, String> {
        let logger = log::LOG.read().unwrap().new(o!());

        let mut mjcf_model = MJCFModel {
            model_name: String::from("MuJoCo Model"),
            shapes: HashMap::new(),
            colliders: HashMap::new(),
            materials: HashMap::new(),
        };

        debug!(logger, "Parsing XML string");
        let doc = match roxmltree::Document::parse(text) {
            Ok(doc) => doc,
            Err(error) => {
                error!(logger, "Failed to parse XML string. Reason: {:?}", error);
                return Err(format!("{:?}", error));
            }
        };

        let root = doc.root_element();

        // TODO(dschwab): change this to a proper error
        if !root.has_tag_name("mujoco") {
            error!(
                logger,
                "Non 'mujoco' root tag.";
                "root tag name" => root.tag_name().name());
            return Err(format!(
                "Non 'mujoco' root tag. Got {}",
                root.tag_name().name()
            ));
        }
        if let Some(model_name) = root.attribute("model") {
            mjcf_model.model_name = model_name.to_string();
            debug!(logger, "Changed model name";
                   "model_name" => &mjcf_model.model_name);
        }

        for child in root.children() {
            match child.tag_name().name() {
                "worldbody" => mjcf_model.parse_worldbody(&logger, &child)?,
                _ => {}
            };
        }

        Ok(mjcf_model)
    }

    fn parse_worldbody(
        &mut self,
        logger: &slog::Logger,
        worldbody_node: &roxmltree::Node,
    ) -> Result<(), String> {
        debug!(logger, "Parsing worldbody tag");
        if !worldbody_node.attributes().is_empty() {
            error!(logger, "worldbody has attributes";
                   "worldbody attributes" => ?worldbody_node.attributes());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
