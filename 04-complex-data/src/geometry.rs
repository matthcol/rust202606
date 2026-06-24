#[derive(Debug)]
pub struct Point{
    pub x: f64,
    pub y: f64,
    pub name: Option<String>
}

impl Point {
    pub fn distance(&self, other: &Self) -> f64 {
        (self.x - other.x).hypot(self.y - other.y)
    }
    
}

#[cfg(test)]
mod tests{
    use rstest::{fixture, rstest};
    use super::Point;

    #[fixture]
    fn point_a() -> Point{
        Point { x: 3.5, y: 7.25, name: Some(String::from("A")) }
    }

    #[rstest]
    fn test_point_distance_self(point_a: Point){
        let d = point_a.distance(&point_a);
        assert_eq!(0., d)
    }

}