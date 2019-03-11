use failure::Fail;
use na::Real;
use nalgebra as na;
use ncollide3d::shape;
use ncollide3d::shape::ShapeHandle;
use nphysics3d::object::ColliderDesc;
use roxmltree;
use slog::{debug, warn};

#[derive(Clone, PartialEq, Debug, Fail)]
pub enum GeomError {
    #[fail(display = "Invalid shape type {}", geom_type)]
    InvalidType { geom_type: String },
    #[fail(display = "Geom type {} is not currently supported", geom_type)]
    UnsupportedType { geom_type: String },
}

pub fn parse_geom_node<N: na::Real>(
    logger: &slog::Logger,
    geom_node: &roxmltree::Node,
) -> Result<ColliderDesc<N>, GeomError> where N: From<f32> {
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
            warn!(logger, "Size currently ignored"; "type" => "sphere");
            ShapeHandle::new(shape::Ball::new(N::from(0.1)))
        }
        Some("capsule") => {
            warn!(logger, "Size currently signored"; "type" => "capsule");
            ShapeHandle::new(shape::Capsule::new(N::from(0.5), N::from(0.2)))
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
            warn!(logger, "Size currently ignored"; "type" => "box");
            ShapeHandle::new(shape::Cuboid::new(na::Vector3::repeat(N::from(1.0))))
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

    let mut collider_desc = ColliderDesc::new(shape_handle);

    if let Some(name) = geom_node.attribute("name") {
        collider_desc.set_name(name.to_owned());
    }

    Ok(collider_desc)
}
