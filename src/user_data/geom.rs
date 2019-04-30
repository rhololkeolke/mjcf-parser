use nalgebra as na;
use nphysics_user_data_traits::HasColor;

#[derive(Clone, PartialEq, Debug)]
pub struct GeomUserData<N: na::RealField> {
    pub torsional_friction: N,
    pub rolling_friction: N,
    pub rgba: na::Point4<f32>,
}

impl<N: na::RealField> Default for GeomUserData<N>
where
    N: From<f32>,
{
    fn default() -> GeomUserData<N> {
        GeomUserData {
            torsional_friction: N::from(0.005),
            rolling_friction: N::from(0.0001),
            rgba: na::Point4::new(0.5, 0.5, 0.5, 1.0),
        }
    }
}

impl<N: na::RealField> HasColor for GeomUserData<N> {
    fn color(&self) -> na::Point3<f32> {
        na::Point3::new(self.rgba.x, self.rgba.y, self.rgba.z)
    }
}
