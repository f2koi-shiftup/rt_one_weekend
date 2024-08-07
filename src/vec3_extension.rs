use gfxmath_vec3::{ops::Dot, vec3, Vec3};

#[allow(dead_code)]
pub trait Vec3Extension<T>
where
    Vec3<T>: Dot<Output = T>,
{
    fn unit_x() -> Vec3<T>;
    fn unit_y() -> Vec3<T>;
    fn unit_z() -> Vec3<T>;
    fn zero() -> Vec3<T>;
    fn squared(&self) -> T;
    fn l2norm(&self) -> T;
}

impl Vec3Extension<f32> for Vec3<f32> {
    fn unit_x() -> Vec3<f32> {
        vec3!(1.0, 0.0, 0.0)
    }

    fn unit_y() -> Vec3<f32> {
        vec3!(0.0, 1.0, 0.0)
    }

    fn unit_z() -> Vec3<f32> {
        vec3!(0.0, 0.0, 1.0)
    }

    fn zero() -> Vec3<f32> {
        vec3!(0.0, 0.0, 0.0)
    }

    fn squared(&self) -> f32 {
        self.dot(self)
    }

    fn l2norm(&self) -> f32 {
        self.squared().sqrt()
    }
}
