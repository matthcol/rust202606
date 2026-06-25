
use std::f64;

trait Mesurable2D{
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

#[derive(Debug)]
struct Point(f64, f64);

#[derive(Debug)]
struct Circle{
    radius: f64,
    center: Point
}

impl Mesurable2D for Circle{
    fn area(&self) -> f64 {
        self.radius.powi(2) * f64::consts::PI
    }

    fn perimeter(&self) -> f64 {
        self.radius * 2. * f64::consts::PI
    }
}

#[derive(Debug)]
struct Polygon{}

impl Mesurable2D for Polygon {
    fn area(&self) -> f64 {
        1.
    }

    fn perimeter(&self) -> f64 {
        2.
    }
}

fn total_area_slice_circle(slice: &[Circle]) -> f64{
    slice.iter()
        .map(Circle::area)
        .sum::<f64>()
}

fn total_area_slice_mesurable<M>(slice: &[M]) -> f64
where
    M: Mesurable2D
{
    slice.iter()
        .map(Mesurable2D::area)
        .sum::<f64>()
}

fn total_area_iterator_mesurable<'a, I, M>(iterator: I) -> f64
where
    I: Iterator<Item=&'a M>,
    M: Mesurable2D + 'a
{
    iterator
        .map(|m| m.area())
        .sum::<f64>()
}

fn total_area_iterator_dyn_mesurable<'a, I>(iterator: I) -> f64
where
    I: Iterator<Item=&'a dyn Mesurable2D>
{
    iterator
        .map(|m| m.area())
        .sum::<f64>()
}


fn play_with_mesurables(){
    let circles = vec![
        Circle{
            radius: 1.,
            center: Point(3., 5.)
        },
        Circle{
            radius: 1.5,
            center: Point(3., 5.)
        },
        Circle{
            radius: 7.25,
            center: Point(3., 5.)
        },
        Circle{
            radius: 4.,
            center: Point(3., 5.)
        },
    ];
    let polygons = vec![Polygon{}, Polygon{}];
    let mut mesurables: Vec<&dyn Mesurable2D> = Vec::new();
    circles.iter()
        .for_each(|c| mesurables.push(c));
     polygons.iter()
        .for_each(|p| mesurables.push(p));
        
    println!("{circles:#?}");
    println!("{polygons:#?}");
    // pipeline direct
    let total_area: f64 = circles.iter()
        .map(Circle::area)
        .sum();
    println!("Circles total area: {}", total_area.ceil());

    let total_area: f64 = total_area_slice_circle(&circles);
    println!("Circles total area: {}", total_area.ceil());

    let total_area: f64 = total_area_slice_mesurable(&circles);
    println!("Circles total area: {}", total_area.ceil());

    let total_area: f64 = total_area_iterator_mesurable(circles.iter());
    println!("Circles total area: {}", total_area.ceil());

    let total_area = total_area_iterator_mesurable(polygons.iter());
    println!("Polygon total area: {}", total_area.ceil());

    let total_area = total_area_iterator_dyn_mesurable(mesurables.into_iter());
    println!("Mesurable total area: {}", total_area.ceil());
}

fn main() {
    play_with_mesurables();
}
