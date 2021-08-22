use super::rtweekend::clamp;

#[derive(Clone, Copy)]
pub struct Vec3 {
    pub e : [f64; 3],
}

impl Vec3 {
    pub fn new(x:f64, y:f64, z:f64) -> Self {
        Vec3 {
            e : [x, y, z]
        }
    }
    pub fn x(&self) -> f64 { self.e[0] }
    pub fn y(&self) -> f64 { self.e[1] }
    pub fn z(&self) -> f64 { self.e[2] }

    pub fn length(&self) -> f64 { self.length_squared().sqrt() }
    pub fn length_squared(&self) -> f64 {
        self.e[0]*self.e[0]
        +self.e[1]*self.e[1]
        +self.e[2]*self.e[2]
    }

    pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
        u.e[0] * v.e[0]
        +u.e[1] * v.e[1]
        +u.e[2] * v.e[2]
    }

    pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
        Vec3 {
            e: [
                u.e[1] * v.e[2] - u.e[2] * v.e[1],
                u.e[2] * v.e[0] - u.e[0] * v.e[2],
                u.e[0] * v.e[1] - u.e[1] * v.e[0]
            ]
        }
    }

    pub fn unit_vector(v: &Vec3) -> Vec3 {
        v.clone() / v.length()
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Vec3 {
            e: [-self.e[0], -self.e[1], -self.e[2]]
        }
    }
}

impl std::ops::Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            e:  [self.e[0]+rhs.e[0],
                 self.e[1]+rhs.e[1],
                 self.e[2]+rhs.e[2]]
        }
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            e:  [self.e[0]-rhs.e[0],
                 self.e[1]-rhs.e[1],
                 self.e[2]-rhs.e[2]]
        }
    }
}

impl std::ops::Mul for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Vec3 {
            e:  [self.e[0]*rhs.e[0],
                 self.e[1]*rhs.e[1],
                 self.e[2]*rhs.e[2]]
        }
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            e:  [self.e[0]*rhs,
                 self.e[1]*rhs,
                 self.e[2]*rhs]
        }
    }
}

impl std::ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl std::ops::Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        (1.0/rhs) * self
    }
}

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}

impl std::ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
    }
}

impl std::ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0/rhs;
    }
}

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Color {
    pub fn write_color(pixel_color : Color, samples_per_pixel: i32) {
        let r = pixel_color.x();
        let g = pixel_color.y();
        let b = pixel_color.z();

        let scale = 1.0 / samples_per_pixel as f64;
        let r = r * scale;
        let g = g * scale;
        let b = b * scale;

        println!("{} {} {}" , (256.0 * clamp(r, 0.0, 0.999)) as i32
                            , (256.0 * clamp(g, 0.0, 0.999)) as i32
                            , (256.0 * clamp(b, 0.0, 0.999)) as i32);
    }
}