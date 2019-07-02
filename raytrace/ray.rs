mod vec3;
use crate::{
    vec3::{Vec3},
};

pub struct Ray {
    a: Ray,
    b: Ray,
}

impl Ray {
    pub fn new(a: Vec3, b: Vec3) -> Self {
        Ray { a, b }
    }

    pub fn origin() -> Vec3 {
        a
    }
    pub fn direction() -> Vec3 {
        b
    }
    pub fn point_at_parameter(t: f32) {
        a + (b*t)
    }
}
