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

    #[allow(unused)]  // test only, pas d'utilisation dans l'appli...
    pub fn translate(&mut self, delta_x: f64, delta_y: f64){
        self.x += delta_x;
        self.y += delta_y
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


    // distance : point_a + others case:  (x, y, expected_distance)
    #[rstest]
    #[case(7.5, 4.25, 5., 0.)]
    #[case(43.5, 37.25, 50., 0.)]
    #[case(3.8, 7.65, 0.5, 1E-15)]
    fn test_point_distance_other(
        point_a: Point, 
        #[case] x_o: f64, 
        #[case] y_o: f64, 
        #[case] expected_distance: f64,
        #[case] epsilon: f64
    ){
        use approx::assert_relative_eq;

        let point_o = Point{x: x_o, y: y_o, name: None};
        let d = point_a.distance(&point_o);
        assert_relative_eq!(expected_distance, d, epsilon=epsilon)
    }

    #[rstest]
    fn test_point_translate(mut point_a: Point){
        point_a.translate(1.25, -1.25);
        assert_eq!(4.75, point_a.x);
        assert_eq!(6.0, point_a.y);
    }
}