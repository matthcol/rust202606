#[derive(Debug)]
pub struct Point{
    pub x: f64,
    pub y: f64,
    pub name: String
}

impl Point {
    pub fn distance(&self, other: &Self) -> f64 {
        (self.x - other.x).hypot(self.y - other.y)
    }
    
}

