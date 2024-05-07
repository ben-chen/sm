use fixed::{traits::ToFixed, types::extra::U7, FixedI32};
use sdl2::rect::Point;
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// Fixed-point number with 7 bits of fractional precision
/// This is used for physics calculations to avoid floating point errors
pub type Fi32 = FixedI32<U7>;

/// Fixed-point point with 7 bits of fractional precision
/// This is used for physics calculations to avoid floating point errors
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash)]
pub struct PointFi32 {
    pub x: Fi32,
    pub y: Fi32,
}

impl PointFi32 {
    pub fn new<Src1, Src2>(x: Src1, y: Src2) -> Self
    where
        Src1: ToFixed,
        Src2: ToFixed,
    {
        PointFi32 {
            x: Fi32::from_num(x),
            y: Fi32::from_num(y),
        }
    }

    pub fn offset(&self, dx: Fi32, dy: Fi32) -> Self {
        PointFi32 {
            x: self.x + Fi32::from_num(dx),
            y: self.y + Fi32::from_num(dy),
        }
    }

    pub fn dot(&self, other: Self) -> Fi32 {
        self.x * other.x + self.y * other.y
    }

    pub fn normalize(self) -> PointFi32 {
        let l = (self.x * self.x + self.y * self.y).sqrt();
        if l.is_zero() {
            return PointFi32::new(Fi32::ZERO, Fi32::ZERO);
        }
        PointFi32::new(self.x / l, self.y / l)
    }
}

impl From<Point> for PointFi32 {
    fn from(point: Point) -> Self {
        PointFi32 {
            x: Fi32::from_num(point.x),
            y: Fi32::from_num(point.y),
        }
    }
}

impl From<PointFi32> for Point {
    fn from(point: PointFi32) -> Self {
        Point::new(point.x.to_num(), point.y.to_num())
    }
}

impl Neg for PointFi32 {
    type Output = Self;

    fn neg(self) -> Self {
        PointFi32 {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Add for PointFi32 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        PointFi32 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for PointFi32 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Sub for PointFi32 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        PointFi32 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl SubAssign for PointFi32 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl Mul<Fi32> for PointFi32 {
    type Output = Self;

    fn mul(self, rhs: Fi32) -> Self {
        PointFi32 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl MulAssign<Fi32> for PointFi32 {
    fn mul_assign(&mut self, rhs: Fi32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}
