use nalgebra as na;

#[derive(Clone, PartialEq, Debug)]
pub struct GeomUserData<N: na::Real> {
    pub torsional_friction: N,
    pub rolling_friction: N,
    pub rgba: na::Point4<f32>,
}

impl<N: na::Real> Default for GeomUserData<N>
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
