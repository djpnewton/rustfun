use std::ops;

pub struct Vec3 {
    e0: f32,
    e1: f32,
    e2: f32,
}

impl Vec3 {
    pub fn new(e0: f32, e1: f32, e2: f32) -> Self {
        Vec3 { e0, e1, e2 }
    }

    pub fn x(&self) -> f32 {
        self.e0
    }
    pub fn y(&self) -> f32 {
        self.e1
    }
    pub fn z(&self) -> f32 {
        self.e2
    }
    pub fn r(&self) -> f32 {
        self.e0
    }
    pub fn g(&self) -> f32 {
        self.e1
    }
    pub fn b(&self) -> f32 {
        self.e2
    }

    pub fn length(&self) -> f32 {
        (self.e0 * self.e0 + self.e1 * self.e1 + self.e2 * self.e2).sqrt()
    }
    pub fn squared_length(&self) -> f32 {
        self.e0 * self.e0 + self.e1 * self.e1 + self.e2 * self.e2
    }
    pub fn make_unit_vector(&mut self) {
        let k = 1.0 / self.length();
        self.e0 *= k;
        self.e1 *= k;
        self.e2 *= k;
        ()
    }

    pub fn dot(&self, v: Vec3) -> f32 {
        self.e0 * v.e0 + self.e1 * v.e1 + self.e2 * v.e2
    }
    pub fn cross(&self, v: Vec3) -> Vec3 {
        Vec3::new(self.e1 * v.e2 - self.e2 * v.e1,
                  -(self.e0 * v.e2 - self.e2 * v.e0),
                  self.e0 * v.e1 - self.e1 * v.e0)
    }

    pub fn unit_vector(&self) -> Vec3 {
        self / self.length()
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3::new(-self.e0, -self.e1, -self.e2)
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, _rhs: Vec3) -> Vec3 {
        Vec3::new(self.e0 + _rhs.e0, self.e1 + _rhs.e1, self.e2 + _rhs.e2)
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, _rhs: Vec3) -> Vec3 {
        Vec3::new(self.e0 - _rhs.e0, self.e1 - _rhs.e1, self.e2 - _rhs.e2)
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, _rhs: Vec3) -> Vec3 {
        Vec3::new(self.e0 * _rhs.e0, self.e1 * _rhs.e1, self.e2 * _rhs.e2)
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, _rhs: f32) -> Vec3 {
        Vec3::new(self.e0 * _rhs, self.e1 * _rhs, self.e2 * _rhs)
    }
}

impl ops::Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, _rhs: Vec3) -> Vec3 {
        Vec3::new(self.e0 / _rhs.e0, self.e1 / _rhs.e1, self.e2 / _rhs.e2)
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, _rhs: f32) -> Vec3 {
        Vec3::new(self.e0 / _rhs, self.e1 / _rhs, self.e2 / _rhs)
    }
}

impl ops::Div<f32> for &Vec3 {
    type Output = Vec3;

    fn div(self, _rhs: f32) -> Vec3 {
        Vec3::new(self.e0 / _rhs, self.e1 / _rhs, self.e2 / _rhs)
    }
}
