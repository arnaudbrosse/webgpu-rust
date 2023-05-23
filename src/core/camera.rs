use crate::math::vector3::Vector3;

pub struct Camera {
    pub position: Vector3,
    pub target: Vector3,
    pub up: Vector3,
    pub aspect_ratio: f32,
    pub fov: f32,
    pub near: f32,
    pub far: f32,
}

impl Camera {
    pub fn new(
        position: Vector3,
        target: Vector3,
        up: Vector3,
        aspect_ratio: f32,
        fov: f32,
        near: f32,
        far: f32,
    ) -> Self {
        Camera {
            position,
            target,
            up,
            aspect_ratio,
            fov,
            near,
            far,
        }
    }
}
