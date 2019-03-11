use crate::error::{MJCFParseError, MJCFParseErrorKind, MJCFParseResult};
use crate::log;
use crate::tags;
use na::Real;
use nalgebra as na;
use ncollide3d::shape::ShapeHandle;
use nphysics3d::material::MaterialHandle;
use nphysics3d::object::ColliderDesc;
use nphysics3d::world::World;
use roxmltree;
use slog::{debug, info, o, warn};
use std::collections::HashMap;
use std::str::FromStr;

pub struct MJCFModel<N: Real> {
    pub model_name: String,
    shapes: HashMap<String, ShapeHandle<N>>,
    colliders: HashMap<String, ColliderDesc<N>>,
    materials: HashMap<String, MaterialHandle<N>>,
}

impl<N: Real> MJCFModel<N>
where
    N: From<f32>,
    N: FromStr,
    <N as FromStr>::Err: std::fmt::Display,
{
    // TODO(dschwab): proper return type and error type
    pub fn parse_xml_string(text: &str) -> MJCFParseResult<MJCFModel<N>> {
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
                return Err(MJCFParseError::from(MJCFParseErrorKind::BadXML(format!(
                    "{}",
                    error
                ))));
            }
        };

        let root = doc.root_element();

        // TODO(dschwab): change this to a proper error
        if !root.has_tag_name("mujoco") {
            return Err(MJCFParseError::from(
                MJCFParseErrorKind::MissingRequiredTag {
                    tag_name: String::from("mujoco"),
                },
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
    ) -> Result<(), MJCFParseError> {
        debug!(logger, "Parsing worldbody tag");
        if !worldbody_node.attributes().is_empty() {
            return Err(MJCFParseError::from(
                MJCFParseErrorKind::WorldBodyHasAttributes,
            ));
        }

        for child in worldbody_node.children() {
            match child.tag_name().name() {
                "inertial" | "joint" | "freejoint" => {
                    return Err(MJCFParseError::from(
                        MJCFParseErrorKind::WorldBodyInvalidChildren,
                    ));
                }
                "body" => {} // TODO(dschwab): Parse me
                "geom" => {
                    tags::geom::parse_geom_node::<N>(logger, &child)?;

                    ()
                }
                "site" => {}   // TODO(dschwab): Parse me
                "camera" => {} // TODO(dschwab): Parse me
                "light" => {}  // TODO(dschwab): Parse me
                tag => warn!(logger, "Ignorning unsupported tag"; "tag" => tag),
            };
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_malformed_xml() {
        let bad_xml = "<mujoco";

        let model_result = MJCFModel::<f32>::parse_xml_string(bad_xml);
        match model_result {
            Err(error) => match error.kind() {
                MJCFParseErrorKind::BadXML(_) => {}
                _ => panic!("Got unexpected error type {}", error),
            },
            _ => panic!("Model parsed successfully with bad xml"),
        };
    }

    #[test]
    fn parse_missing_mujoco_tag() {
        let missing_mujoco_tag = "<foo></foo>";

        let model_result = MJCFModel::<f32>::parse_xml_string(missing_mujoco_tag);
        match model_result {
            Err(error) => match error.kind() {
                MJCFParseErrorKind::MissingRequiredTag { tag_name } => {
                    assert_eq!(tag_name, "mujoco")
                }
                _ => panic!("Got unexpected error type {}", error),
            },
            _ => panic!("Model parse successfully when missing mujoco tag"),
        };
    }

    #[test]
    fn worldbody_has_attributes() {
        let xml = "<mujoco><worldbody name=\"This is illegal\"></worldbody><mujoco>";

        let model_result = MJCFModel::<f32>::parse_xml_string(xml);
        match model_result {
            Err(error) => match error.kind() {
                MJCFParseErrorKind::WorldBodyHasAttributes => {}
                _ => panic!("Got unexpected error type {}", error),
            },
            _ => panic!("Model parse successfully when worldbody has attributes"),
        };
    }

    #[test]
    fn worldbody_inertial_child_is_invalid() {
        let xml = "<mujoco><worldbody><inertial></inertial></worldbody></mujoco>";

        let model_result = MJCFModel::<f32>::parse_xml_string(xml);
        match model_result {
            Err(error) => match error.kind() {
                MJCFParseErrorKind::WorldBodyInvalidChildren => {}
                _ => panic!("Got unexpected error type {}", error),
            },
            _ => panic!("Model parse successfully when worldbody has inertial child"),
        };
    }

    #[test]
    fn worldbody_joint_child_is_invalid() {
        let xml = "<mujoco><worldbody><joint></joint></worldbody></mujoco>";

        let model_result = MJCFModel::<f32>::parse_xml_string(xml);
        match model_result {
            Err(error) => match error.kind() {
                MJCFParseErrorKind::WorldBodyInvalidChildren => {}
                _ => panic!("Got unexpected error type {}", error),
            },
            _ => panic!("Model parse successfully when worldbody has joint child"),
        };
    }

    #[test]
    fn worldbody_freejoint_child_is_invalid() {
        let xml = "<mujoco><worldbody><freejoint></freejoint></worldbody></mujoco>";

        let model_result = MJCFModel::<f32>::parse_xml_string(xml);
        match model_result {
            Err(error) => match error.kind() {
                MJCFParseErrorKind::WorldBodyInvalidChildren => {}
                _ => panic!("Got unexpected error type {}", error),
            },
            _ => panic!("Model parse successfully when worldbody has freejoint child"),
        };
    }
}
