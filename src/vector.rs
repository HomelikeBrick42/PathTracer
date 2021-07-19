#[derive(Clone, Copy)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn dot(a: &Vector3, b: &Vector3) -> f64 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    pub fn cross(a: &Vector3, b: &Vector3) -> Vector3 {
        Vector3 {
            x: a.y * b.z - a.z * b.y,
            y: a.z * b.x - a.x * b.z,
            z: a.x * b.y - a.y * b.x,
        }
    }

    pub fn normalized(&self) -> Self {
        let length = Vector3::dot(self, self).sqrt();
        if length > 0.0 {
            return *self / length;
        }
        return Vector3::new(0.0, 0.0, 0.0);
    }
}

impl std::ops::Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::Add<Vector3> for f64 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self + other.x,
            y: self + other.y,
            z: self + other.z,
        }
    }
}

impl std::ops::Add<f64> for Vector3 {
    type Output = Vector3;

    fn add(self, other: f64) -> Vector3 {
        Vector3 {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        }
    }
}

impl std::ops::AddAssign<f64> for &mut Vector3 {
    fn add_assign(&mut self, other: f64) {
        self.x += other;
        self.y += other;
        self.z += other;
    }
}

impl std::ops::AddAssign<Vector3> for &mut Vector3 {
    fn add_assign(&mut self, other: Vector3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl std::ops::Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::ops::Sub<Vector3> for f64 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self - other.x,
            y: self - other.y,
            z: self - other.z,
        }
    }
}

impl std::ops::Sub<f64> for Vector3 {
    type Output = Vector3;

    fn sub(self, other: f64) -> Vector3 {
        Vector3 {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
        }
    }
}

impl std::ops::SubAssign<f64> for &mut Vector3 {
    fn sub_assign(&mut self, other: f64) {
        self.x -= other;
        self.y -= other;
        self.z -= other;
    }
}

impl std::ops::SubAssign<Vector3> for &mut Vector3 {
    fn sub_assign(&mut self, other: Vector3) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl std::ops::Mul<Vector3> for Vector3 {
    type Output = Vector3;

    fn mul(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl std::ops::Mul<Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}

impl std::ops::Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, other: f64) -> Vector3 {
        Vector3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl std::ops::MulAssign<f64> for &mut Vector3 {
    fn mul_assign(&mut self, other: f64) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}

impl std::ops::MulAssign<Vector3> for &mut Vector3 {
    fn mul_assign(&mut self, other: Vector3) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}

impl std::ops::Div<Vector3> for Vector3 {
    type Output = Vector3;

    fn div(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl std::ops::Div<Vector3> for f64 {
    type Output = Vector3;

    fn div(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self / other.x,
            y: self / other.y,
            z: self / other.z,
        }
    }
}

impl std::ops::Div<f64> for Vector3 {
    type Output = Vector3;

    fn div(self, other: f64) -> Vector3 {
        Vector3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl std::ops::DivAssign<f64> for &mut Vector3 {
    fn div_assign(&mut self, other: f64) {
        self.x /= other;
        self.y /= other;
        self.z /= other;
    }
}

impl std::ops::DivAssign<Vector3> for &mut Vector3 {
    fn div_assign(&mut self, other: Vector3) {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
    }
}
