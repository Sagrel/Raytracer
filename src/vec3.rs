use std::ops;

use rand::{Rng, thread_rng};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn zero() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn dot(a: Vec3, b: Vec3) -> f32 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    pub fn cross(a: Vec3, b: Vec3) -> Vec3 {
        Vec3::new(
            a.y * b.z - a.z * b.y,
            a.z * b.x - a.x * b.z,
            a.x * b.y - a.y * b.x,
        )
    }

    pub fn reflect(a: Vec3, b: Vec3) -> Vec3 {
        a - b * Vec3::dot(a, b) * 2.0
    }

    pub fn refract(a: Vec3, b: Vec3, ni_over_nt: f32) -> Option<Vec3> {
        let v = a.normalized();
        let dt = Vec3::dot(a, b);
        let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);

        if discriminant <= 0.0 {
            None
        } else {
            Some((v - b * dt) * ni_over_nt - b * discriminant.sqrt())
        }
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        let mut v = Vec3::new(5.5, 5.5, 5.5);
        let vec1 = Vec3::new(1.0, 1.0, 1.0);

        let mut rng = thread_rng();
        while v.length() >= 1.0 {
            v = Vec3::new(rng.gen(), rng.gen(), rng.gen()) * 2.0 - vec1;
        }

        v
    }

    /*
    pub fn rotate_x(&self, a: f32) -> Vec3 {
        Vec3::new(
            self.x,
            self.y * a.cos() - self.z * a.sin(),
            self.y * a.sin() + self.z * a.cos(),
        )
    }

    pub fn rotate_y(&self, a: f32) -> Vec3 {
        Vec3::new(
            self.x * a.cos() + self.z * a.sin(),
            self.y,
            -self.x * a.sin() + self.z * a.cos(),
        )
    }
    */

    pub fn length(self) -> f32 {
        Vec3::dot(self, self).sqrt()
    }

    pub fn normalized(self) -> Vec3 {
        self / self.length()
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, _rhs: Vec3) -> Vec3 {
        Vec3::new(self.x + _rhs.x, self.y + _rhs.y, self.z + _rhs.z)
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, _rhs: Vec3) -> Vec3 {
        Vec3::new(self.x - _rhs.x, self.y - _rhs.y, self.z - _rhs.z)
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, _rhs: Vec3) -> Vec3 {
        Vec3::new(self.x * _rhs.x, self.y * _rhs.y, self.z * _rhs.z)
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, _rhs: f32) -> Vec3 {
        Vec3::new(self.x * _rhs, self.y * _rhs, self.z * _rhs)
    }
}

impl ops::Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, _rhs: Vec3) -> Vec3 {
        Vec3::new(self.x / _rhs.x, self.y / _rhs.y, self.z / _rhs.z)
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, _rhs: f32) -> Vec3 {
        Vec3::new(self.x / _rhs, self.y / _rhs, self.z / _rhs)
    }
}
