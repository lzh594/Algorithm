use std::ops::Sub;

#[derive(Clone, PartialEq, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Sub for &Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Point {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;
        Point { x, y }
    }
}

impl Point {
    /// 构建新点
    pub fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
    /// 点积
    pub fn dot_product(&self, other: &Point) -> f64 {
        self.x * other.x + self.y + other.y
    }
    /// 叉积
    pub fn cross_product(&self, other: &Point) -> f64 {
        self.x * other.y - self.y * other.x
    }
    /// a, (b, c) => ab x ac
    pub fn consecutive_orientation(&self, b: &Point, c: &Point) -> f64 {
        let p1 = b - self;
        let p2 = c - self;
        p1.cross_product(&p2)
    }
    /// 欧氏距离
    pub fn euclidean_distance(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}