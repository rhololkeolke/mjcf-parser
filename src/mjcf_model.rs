use crate::error::{MJCFParseError, MJCFParseErrorKind, MJCFParseResult};
use crate::log;
use na::Real;
use nalgebra as na;
use ncollide3d::shape::ShapeHandle;
use nphysics3d::material::MaterialHandle;
use nphysics3d::object::ColliderDesc;
use nphysics3d::world::World;
use roxmltree;
use slog::{debug, info, o, warn};
use std::collections::HashMap;

pub struct MJCFModel<N: Real> {
    pub model_name: String,
    shapes: HashMap<String, ShapeHandle<N>>,
    colliders: HashMap<String, ColliderDesc<N>>,
    materials: HashMap<String, MaterialHandle<N>>,
}

impl<N: Real> MJCFModel<N> {
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
                "geom" => self.parse_geom(logger, &child)?,
                "site" => {}   // TODO(dschwab): Parse me
                "camera" => {} // TODO(dschwab): Parse me
                "light" => {}  // TODO(dschwab): Parse me
                tag => warn!(logger, "Ignorning unsupported tag"; "tag" => tag),
            };
        }

        Ok(())
    }

    fn parse_geom(
        &mut self,
        logger: &slog::Logger,
        geom_node: &roxmltree::Node,
    ) -> Result<(), MJCFParseError> {
        debug!(logger, "Parsing geom tag");

        let name = match geom_node.attribute("name") {
            Some(name) => name.to_string(),
            None => format!("{}", self.shapes.len()),
        };

        if geom_node.has_attribute("class") {
            warn!(logger, "class attribute is currently unspported"; "node" => ?geom_node);
        }

        let shape_handle = match geom_node.attribute("type") {
            Some("plane") => {}         // TODO(dschwab): implement
            Some("hfield") => {}        // TODO(dschwab): implement
            Some("sphere") | None => {} // TODO(dschwab): implement
            Some("capsule") => {}       // TODO(dschwab): implement
            Some("ellipsoid") => {}     // TODO(dschwab): implement
            Some("cylinder") => {}      // TODO(dschwab): implement
            Some("box") => {}           // TODO(dschwab): implement
            Some("mesh") => {}          // TODO(dschwab): implement
            Some(geom_type) => {
                return Err(MJCFParseError::from(MJCFParseErrorKind::InvalidGeomType {
                    geom_type: geom_type.to_string(),
                }));
            }
        };

        if geom_node.has_attribute("contype") {
            warn!(logger, "contype attribute is currently unsupported"; "node" => ?geom_node);
        }

        if geom_node.has_attribute("conaffinity") {
            warn!(logger, "conaffinity attribute is currently unsupported"; "node" => ?geom_node);
        }

        if geom_node.has_attribute("condim") {
            warn!(logger, "condim attribute is currently unsupported"; "node" => ?geom_node);
        }

        if geom_node.has_attribute("group") {
            warn!(logger, "group attribute is currently unsupported"; "node" => ?geom_node);
        }

        if geom_node.has_attribute("priority") {
            warn!(logger, "priority attribute is currently unsupported"; "node" => ?geom_node);
        }

        if geom_node.has_attribute("material") {
            warn!(logger, "material attribute is currently unsupported"; "node" => ?geom_node);
        }

        if geom_node.has_attribute("rgba") {
            warn!(logger, "rgba attribute is currently unsupported"; "node" => ?geom_node);
        }

        if geom_node.has_attribute("friction") {
            warn!(logger, "friction attribute is currently unsupported"; "node" => ?geom_node);
        }

        if geom_node.has_attribute("mass") {
            warn!(logger, "mass attribute is currently unsupported"; "node" => ?geom_node);
        }

        if geom_node.has_attribute("density") {
            warn!(logger, "density attribute is currently unsupported"; "node" => ?geom_node);
        }

        if geom_node.has_attribute("solmix") {
            warn!(logger, "solmix attribute is currently unsupported"; "node" => ?geom_node);
        }

        if geom_node.has_attribute("solref") {
            warn!(logger, "solref attribute is currently unsupported"; "node" => ?geom_node);
        }

        if geom_node.has_attribute("solimpl") {
            warn!(logger, "solimpl attribute is currently unsupported"; "node" => ?geom_node);
        }

        if geom_node.has_attribute("margin") {
            warn!(logger, "margin attribute is currently unsupported"; "node" => ?geom_node);
        }

        if geom_node.has_attribute("gap") {
            warn!(logger, "gap attribute is currently unsupported"; "node" => ?geom_node);
        }

        if geom_node.has_attribute("fromto") {
            warn!(logger, "fromto attribute is currently unsupported"; "node" => ?geom_node);
        }

        if geom_node.has_attribute("pos") {
            warn!(logger, "pos attribute is currently unsupported"; "node" => ?geom_node);
        }

        if geom_node.has_attribute("quat") {
            warn!(logger, "quat attribute is currently unsupported"; "node" => ?geom_node);
        }

        if geom_node.has_attribute("axisangle") {
            warn!(logger, "axisangle attribute is currently unsupported"; "node" => ?geom_node);
        }

        if geom_node.has_attribute("xyaxes") {
            warn!(logger, "xyaxes attribute is currently unsupported"; "node" => ?geom_node);
        }

        if geom_node.has_attribute("zaxis") {
            warn!(logger, "zaxis attribute is currently unsupported"; "node" => ?geom_node);
        }

        if geom_node.has_attribute("euler") {
            warn!(logger, "euler attribute is currently unsupported"; "node" => ?geom_node);
        }

        if geom_node.has_attribute("hfield") {
            warn!(logger, "hfield attribute is currently unsupported"; "node" => ?geom_node);
        }

        if geom_node.has_attribute("mesh") {
            warn!(logger, "mesh attribute is currently unsupported"; "node" => ?geom_node);
        }

        if geom_node.has_attribute("fitscale") {
            warn!(logger, "fitscale attribute is currently unsupported"; "node" => ?geom_node);
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
