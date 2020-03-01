use glam::{Quat, Vec3};

pub struct Transform {
    position: Vec3,
    orientation: Quat,
    scale: Vec3,
}

impl Default for Transform {
    fn default() -> Self {
        Transform {
            position: Vec3::zero(),
            orientation: Quat::identity(),
            scale: Vec3::one(),
        }
    }
}
