use crate::rtweekend::{clamp, random, random_range};

const VEC_EPS: f64 = 1.0e-8;
#[derive(Clone, Copy)]
pub struct Vec3 {
    pub e : [f64; 3],
}

impl Vec3 {
    pub fn new(x:f64, y:f64, z:f64) -> Self {
        Self {
            e : [x, y, z]
        }
    }

    pub fn x(&self) -> f64 { self.e[0] }
    pub fn y(&self) -> f64 { self.e[1] }
    pub fn z(&self) -> f64 { self.e[2] }

    pub fn random() -> Self {
        Self::new(
            random(),
            random(),
            random()
        )
    }
    pub fn random_range(min: f64, max: f64) -> Self {
        Self::new(
            random_range(min, max),
            random_range(min, max),
            random_range(min, max)
        )
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Self::random_range(-1.0, 1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }

    }

    pub fn random_unit_vector() -> Self {
        Self::unit_vector(&Self::random_in_unit_sphere())
    }

    pub fn random_in_hemisphere(normal: &Self) -> Self {
        let in_unit_sphere = Self::random_in_unit_sphere();
        if Self::dot(&in_unit_sphere, &normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    pub fn length(&self) -> f64 { self.length_squared().sqrt() }
    pub fn length_squared(&self) -> f64 {
        self.e[0]*self.e[0] + self.e[1]*self.e[1] + self.e[2]*self.e[2]
    }

    pub fn near_zero(&self) -> bool {
        (self.e[0].abs() < VEC_EPS) && (self.e[1].abs() < VEC_EPS) && (self.e[2].abs() < VEC_EPS)
    }

    pub fn dot(u: &Self, v: &Self) -> f64 {
        u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
    }

    pub fn cross(u: &Self, v: &Self) -> Self {
        Self {
            e: [
                u.e[1] * v.e[2] - u.e[2] * v.e[1],
                u.e[2] * v.e[0] - u.e[0] * v.e[2],
                u.e[0] * v.e[1] - u.e[1] * v.e[0]
            ]
        }
    }

    pub fn reflect(v: &Self, n: &Self) -> Self {
        *v - 2.0 * Self::dot(v, n) * *n
    }

    pub fn unit_vector(v: &Self) -> Self {
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
        Self {
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
        Self {
            e:  [self.e[0]+rhs.e[0],
                 self.e[1]+rhs.e[1],
                 self.e[2]+rhs.e[2]]
        }
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            e:  [self.e[0]-rhs.e[0],
                 self.e[1]-rhs.e[1],
                 self.e[2]-rhs.e[2]]
        }
    }
}

impl std::ops::Mul for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            e:  [self.e[0]*rhs.e[0],
                 self.e[1]*rhs.e[1],
                 self.e[2]*rhs.e[2]]
        }
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
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

        // Divide the color by the number of samples and gamma-correct for gamma=2.0.
        let scale = 1.0 / samples_per_pixel as f64;
        let r = (r * scale).sqrt();
        let g = (g * scale).sqrt();
        let b = (b * scale).sqrt();

        // Write the translated [0, 255] value of each color component.
        println!("{} {} {}" , (256.0 * clamp(r, 0.0, 0.999)) as i32
                            , (256.0 * clamp(g, 0.0, 0.999)) as i32
                            , (256.0 * clamp(b, 0.0, 0.999)) as i32);
    }
}