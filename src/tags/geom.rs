use crate::attributes;
use failure::Fail;
use nalgebra as na;
use ncollide3d::shape;
use ncollide3d::shape::ShapeHandle;
use nphysics3d::object::ColliderDesc;
use roxmltree;
use slog::{debug, warn};
use std::str::FromStr;

#[derive(Clone, PartialEq, Debug, Fail)]
pub enum GeomError {
    #[fail(display = "Invalid shape type {}", geom_type)]
    InvalidType { geom_type: String },
    #[fail(display = "Geom type {} is not currently supported", geom_type)]
    UnsupportedType { geom_type: String },
    #[fail(display = "Required attribute \"{}\" missing", 0)]
    RequiredAttributeMissing(String),
    #[fail(display = "Bad attribute values. {}", 0)]
    BadRealAttribute(#[fail(cause)] attributes::ParseRealAttributeError),
    #[fail(display = "Failed to parse orientation. Reason {}", 0)]
    BadOrientation(#[fail(cause)] attributes::ParseOrientationError),
    #[fail(display = "Multiple positions specified")]
    MultiplePositions,
}

impl From<attributes::ParseRealAttributeError> for GeomError {
    fn from(error: attributes::ParseRealAttributeError) -> GeomError {
        GeomError::BadRealAttribute(error)
    }
}

impl From<attributes::ParseOrientationError> for GeomError {
    fn from(error: attributes::ParseOrientationError) -> GeomError {
        GeomError::BadOrientation(error)
    }
}

pub fn parse_geom_node<N: na::Real>(
    logger: &slog::Logger,
    geom_node: &roxmltree::Node,
) -> Result<ColliderDesc<N>, GeomError>
where
    N: From<f32>,
    N: FromStr,
    <N as FromStr>::Err: std::fmt::Display,
{
    debug!(logger, "Parsing geom tag");

    let shape_handle: ShapeHandle<N> = match geom_node.attribute("type") {
        Some("plane") => {
            warn!(logger, "Size currently ignored"; "type" => "plane");
            warn!(logger, "Orientation currently ignored"; "type" => "plane");
            ShapeHandle::new(shape::Plane::new(na::Unit::new_normalize(na::Vector3::z())))
        }
        Some("hfield") => {
            return Err(GeomError::UnsupportedType {
                geom_type: String::from("hfield"),
            });
        }
        Some("sphere") | None => {
            let size_attr = "size";
            let sizes = match geom_node.attribute(size_attr) {
                Some(size_text) => attributes::parse_real_vector_attribute::<N, na::U1>(size_text)?,
                None => return Err(GeomError::RequiredAttributeMissing(size_attr.to_string())),
            };
            let radius = *sizes.get(0).unwrap();
            ShapeHandle::new(shape::Ball::new(radius))
        }
        Some("capsule") => {
            let size_attr = "size";
            let fromto_attr = "fromto";
            let (half_length, radius) = match geom_node.attribute(size_attr) {
                Some(size_text) => {
                    if geom_node.has_attribute(fromto_attr) {
                        let sizes: na::Vector1<N> =
                            attributes::parse_real_vector_attribute(size_text)?;

                        let radius = *sizes.get(0).unwrap();

                        // parse half length from fromto
                        let fromto: na::Vector6<N> = attributes::parse_real_vector_attribute(
                            geom_node.attribute(fromto_attr).unwrap(),
                        )?;
                        let p0 = fromto.rows(0, 3);
                        let p1 = fromto.rows(3, 3);
                        let half_length = p0.metric_distance(&p1) / N::from(2.0);

                        (half_length, radius)
                    } else {
                        let sizes: na::Vector2<N> =
                            attributes::parse_real_vector_attribute(size_text)?;
                        let radius = *sizes.get(0).unwrap();
                        let half_length = *sizes.get(1).unwrap();

                        (half_length, radius)
                    }
                }
                None => return Err(GeomError::RequiredAttributeMissing(size_attr.to_string())),
            };
            ShapeHandle::new(shape::Capsule::new(half_length, radius))
        }
        Some("ellipsoid") => {
            return Err(GeomError::UnsupportedType {
                geom_type: String::from("ellipsoid"),
            });
        }
        Some("cylinder") => {
            return Err(GeomError::UnsupportedType {
                geom_type: String::from("cylinder"),
            });
        }
        Some("box") => {
            let size_attr = "size";
            let sizes: na::Vector3<N> = match geom_node.attribute(size_attr) {
                Some(size_text) => attributes::parse_real_vector_attribute(size_text)?,
                None => return Err(GeomError::RequiredAttributeMissing(size_attr.to_string())),
            };
            ShapeHandle::new(shape::Cuboid::new(sizes))
        }
        Some("mesh") => {
            return Err(GeomError::UnsupportedType {
                geom_type: String::from("mesh"),
            });
        }
        Some(geom_type) => {
            return Err(GeomError::InvalidType {
                geom_type: geom_type.to_string(),
            });
        }
    };

    let mut collider_desc = ColliderDesc::new(shape_handle);

    if let Some(name) = geom_node.attribute("name") {
        collider_desc.set_name(name.to_owned());
    }

    let translation: na::Translation3<N> = match geom_node.attribute("type") {
        Some("plane") | Some("sphere") | None => match geom_node.attribute("pos") {
            Some(pos) => na::Translation3::from(attributes::parse_real_vector_attribute(pos)?),
            None => na::Translation3::identity(),
        },
        Some("capsule") | Some("box") => match geom_node.attribute("fromto") {
            Some(fromto) => {
                if geom_node.has_attribute("pos") {
                    return Err(GeomError::MultiplePositions);
                } else {
                    // parse half length from fromto
                    let fromto: na::Vector6<N> = attributes::parse_real_vector_attribute(fromto)?;
                    let p0 = na::Point3::from(fromto.fixed_rows::<na::U3>(0).into_owned());
                    let p1 = na::Point3::from(fromto.fixed_rows::<na::U3>(3).into_owned());
                    let dir = na::Vector3::from(p1 - p0);

                    let center: na::Point3<N> = p0 + dir * N::from(0.5);
                    na::Translation3::new(center.x, center.y, center.z)
                }
            }
            None => match geom_node.attribute("pos") {
                Some(pos) => na::Translation3::from(attributes::parse_real_vector_attribute(pos)?),
                None => na::Translation3::identity(),
            },
        },
        Some(geom_type) => {
            return Err(GeomError::InvalidType {
                geom_type: geom_type.to_string(),
            });
        }
    };

    let orientation: na::UnitQuaternion<N> = match geom_node.attribute("type") {
        Some("plane") => attributes::parse_orientation_attribute(logger, geom_node, false)?,
        Some("sphere") | None => attributes::parse_orientation_attribute(logger, geom_node, false)?,
        Some("capsule") => attributes::parse_orientation_attribute(logger, geom_node, true)?,
        Some("box") => attributes::parse_orientation_attribute(logger, geom_node, true)?,
        Some(geom_type) => {
            return Err(GeomError::InvalidType {
                geom_type: geom_type.to_string(),
            });
        }
    };
    collider_desc.set_position(na::Isometry3::from_parts(translation, orientation));

    if geom_node.has_attribute("class") {
        warn!(logger, "class attribute is currently unspported"; "node" => ?geom_node);
    }

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

    if geom_node.has_attribute("hfield") {
        warn!(logger, "hfield attribute is currently unsupported"; "node" => ?geom_node);
    }

    if geom_node.has_attribute("mesh") {
        warn!(logger, "mesh attribute is currently unsupported"; "node" => ?geom_node);
    }

    if geom_node.has_attribute("fitscale") {
        warn!(logger, "fitscale attribute is currently unsupported"; "node" => ?geom_node);
    }

    Ok(collider_desc)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::log;
    use nalgebra as na;
    use proptest::prelude::*;
    use roxmltree;
    use slog::o;

    proptest! {
        #[test]
        fn parse_default_geom_type(ball_radius in proptest::num::f32::NORMAL) {
            prop_assume!(ball_radius != 0.0);

            let xml = format!("<geom size=\"{}\"></geom>", ball_radius);

            let doc = roxmltree::Document::parse(&xml).unwrap();
            let root = doc.root_element();

            let logger = log::LOG.read().unwrap().new(o!());

            let collider_desc = parse_geom_node::<f32>(&logger, &root).unwrap();

            // default is not moved
            prop_assert_eq!(*collider_desc.get_translation(), na::Vector3::zeros());
            // default is sphere with the specified radius
            let ball: &shape::Ball<f32> = collider_desc.get_shape().downcast_ref().unwrap();
            prop_assert_eq!(ball.radius(), ball_radius);

            // TODO(dschwab): test other defaults of the collider desc
        }

        #[test]
        fn parse_bad_sphere_radius(real_values in proptest::collection::vec(proptest::num::f32::NORMAL, 3)) {
            let size_text_attribute = real_values.iter().map(f32::to_string).collect::<Vec<String>>().join(" ");

            let xml = format!("<geom size=\"{}\"></geom>", size_text_attribute);

            let doc = roxmltree::Document::parse(&xml).unwrap();
            let root = doc.root_element();

            let logger = log::LOG.read().unwrap().new(o!());

            if let Err(error) = parse_geom_node::<f32>(&logger, &root) {
                match error {
                    GeomError::BadRealAttribute(_) => {},
                    _ => {
                        return Err(TestCaseError::fail(format!("Unexpected parsing error. {}", error)));
                    }
                }
            } else {
                return Err(TestCaseError::fail("Parsed sphere geom successfully, even with invalid sizes"));
            }

        }

        #[test]
        fn parse_sphere_geom(ball_radius in proptest::num::f32::NORMAL) {
            prop_assume!(ball_radius != 0.0);
            let xml = format!("<geom type=\"sphere\" size=\"{}\"></geom>", ball_radius);

            let doc = roxmltree::Document::parse(&xml).unwrap();
            let root = doc.root_element();

            let logger = log::LOG.read().unwrap().new(o!());

            let collider_desc = parse_geom_node::<f32>(&logger, &root).unwrap();

            // default is not moved
            prop_assert_eq!(*collider_desc.get_translation(), na::Vector3::zeros());
            // default is sphere with the specified radius
            let ball: &shape::Ball<f32> = collider_desc.get_shape().downcast_ref().unwrap();
            prop_assert_eq!(ball.radius(), ball_radius);

            // TODO(dschwab): test other defaults of the collider desc

        }

    }
}
