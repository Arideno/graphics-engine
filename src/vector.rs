use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector { x, y, z }
    }

    pub fn dot(self, other: Vector) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Vector) -> Vector {
        Vector::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x
        )
    }

    pub fn len(self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn len_sq(self) -> f64 {
        self.dot(self)
    }

    pub fn normalize(self) -> Vector {
        self / self.len()
    }
}

impl From <(f64, f64, f64)> for Vector {
    fn from (tuple: (f64, f64, f64)) -> Vector {
        Vector { x: tuple.0, y: tuple.1, z: tuple.2 }
    }
}

impl Add<Vector> for Vector {
    type Output = Vector;
    fn add(self, rhs: Vector) -> Self::Output {
        Vector::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl AddAssign<Vector> for Vector {
    fn add_assign(&mut self, rhs: Vector) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub<Vector> for Vector {
    type Output = Vector;
    fn sub(self, rhs: Vector) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl SubAssign<Vector> for Vector {
    fn sub_assign(&mut self, rhs: Vector) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;
    fn mul(self, rhs: f64) -> Self::Output {
        Vector::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul<Vector> for f64 {
    type Output = Vector;
    fn mul(self, rhs: Vector) -> Self::Output {
        Vector::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

impl MulAssign<f64> for Vector {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Div<f64> for Vector {
    type Output = Vector;
    fn div(self, rhs: f64) -> Self::Output {
        Vector::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl DivAssign<f64> for Vector {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl Neg for Vector {
    type Output = Vector;
    fn neg(self) -> Self::Output {
        Vector::new(-self.x, -self.y, -self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let vector = Vector::new(5., 4., 3.);
        assert_eq!(5., vector.x);
        assert_eq!(4., vector.y);
        assert_eq!(3., vector.z);
    }   

    #[test]
    fn test_dot() {
        let vector = Vector::new(5., 4., 3.);
        let v1 = Vector::new(1., 2., 3.);
        let v2 = Vector::new(-5., 4., 7.);
        let v3 = Vector::new(1., 1., 1.);
        let result1 = vector.dot(v1);
        let result2 = vector.dot(v2);
        let result3 = vector.dot(v3);
        assert_eq!(22., result1);
        assert_eq!(12., result2);
        assert_eq!(12., result3);
    }   

    #[test]
    fn test_cross() {
        let vector = Vector::new(5., 4., 3.);
        let v1 = Vector::new(1., 2., 3.);
        let v2 = Vector::new(-5., 4., 7.);
        let v3 = Vector::new(1., 1., 1.);
        let result1 = vector.cross(v1);
        let result2 = vector.cross(v2);
        let result3 = vector.cross(v3);
        assert_eq!(Vector::new(6., -12., 6.), result1);
        assert_eq!(Vector::new(16., -50., 40.), result2);
        assert_eq!(Vector::new(1., -2., 1.), result3);
    }   



    #[test]
    fn test_len() {
        let v1 = Vector::new(5., 0., 0.);
        let v2 = Vector::new(-5., 4., 7.);
        let v3 = Vector::new(1., 1., 1.);
        let result1 = v1.len();
        let result2 = v2.len();
        let result3 = v3.len();
        assert_eq!(5., result1);
        assert_eq!((90f64).sqrt(), result2);
        assert_eq!((3f64).sqrt(), result3);
    }   

    #[test]
    fn test_len_sq() {
        let v1 = Vector::new(5., 0., 0.);
        let v2 = Vector::new(-5., 4., 7.);
        let v3 = Vector::new(1., 1., 1.);
        let result1 = v1.len_sq();
        let result2 = v2.len_sq();
        let result3 = v3.len_sq();
        assert_eq!(25., result1);
        assert_eq!(90., result2);
        assert_eq!(3., result3);
    }  

    #[test]
    fn test_normailize() {
        let v1 = Vector::new(5., 0., 0.);
        let v2 = Vector::new(4., 0., 3.);
        let result1 = v1.normalize();
        let result2 = v2.normalize();
        assert_eq!(Vector::new(1., 0., 0.), result1);
        assert_eq!(Vector::new(0.8, 0., 0.6), result2);
      
    }   

    #[test]
    fn test_from() {
        let a = Vector::from((1., 2., 3.));
        assert_eq!(1., a.x);
        assert_eq!(2., a.y);
        assert_eq!(3., a.z);
        let b: Vector = (2., 3., 5.).into();
        assert_eq!(2., b.x);
        assert_eq!(3., b.y);
        assert_eq!(5., b.z);
    }

    #[test]
    fn test_add() {
        let vector = Vector::new(5., 4., 3.);
        let v1 = Vector::new(1., 2., 3.);
        let v2 = Vector::new(-5., 4., 7.);
        let v3 = Vector::new(1., 1., 1.);
        let result1 = vector + v1;
        let result2 = vector + v2;
        let result3 = vector + v3;
        assert_eq!(Vector::new(6., 6., 6.), result1);
        assert_eq!(Vector::new(0., 8., 10.), result2);
        assert_eq!(Vector::new(6., 5., 4.), result3);
    }   


    #[test]
    fn test_add_assign() {
        let mut vector = Vector::new(5., 4., 3.);
        let v1 = Vector::new(1., 2., 3.);
        vector += v1;
        assert_eq!(Vector::new(6., 6., 6.), vector);
    }  

    #[test]
    fn test_sub() {
        let vector = Vector::new(5., 4., 3.);
        let v1 = Vector::new(1., 2., 3.);
        let v2 = Vector::new(-5., 4., 7.);
        let v3 = Vector::new(1., 1., 1.);
        let result1 = vector - v1;
        let result2 = vector - v2;
        let result3 = vector - v3;
        assert_eq!(Vector::new(4., 2., 0.), result1);
        assert_eq!(Vector::new(10., 0., -4.), result2);
        assert_eq!(Vector::new(4., 3., 2.), result3);
    }   


    #[test]
    fn test_sub_assign() {
        let mut vector = Vector::new(5., 4., 3.);
        let v1 = Vector::new(1., 2., 3.);
        vector -= v1;
        assert_eq!(Vector::new(4., 2., 0.), vector);
    }  

    #[test]
    fn test_mul() {
        let vector = Vector::new(5., 4., 3.);
        let result1 = vector * 2.;
        let result2 = 2. * vector;
        assert_eq!(Vector::new(10., 8., 6.), result1);
        assert_eq!(Vector::new(10., 8., 6.), result2);
    }  
    
    #[test]
    fn test_mul_assign() {
        let mut vector = Vector::new(5., 4., 3.);
        vector *= 2.;
        assert_eq!(Vector::new(10., 8., 6.), vector);
    }  

    #[test]
    fn test_dic() {
        let vector = Vector::new(5., 4., 3.);
        let result1 = vector / 2.;
        assert_eq!(Vector::new(2.5, 2., 1.5), result1);
    }  
    
    #[test]
    fn test_div_assign() {
        let mut vector = Vector::new(5., 4., 3.);
        vector /= 2.;
        assert_eq!(Vector::new(2.5, 2., 1.5), vector);
    }  

    #[test]
    fn test_neg() {
        let mut vector = Vector::new(5., 4., 3.);
        vector = -vector;
        assert_eq!(Vector::new(-5., -4., -3.), vector);
    }  
}