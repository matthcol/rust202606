
use std::{any::Any, f64};

trait Mesurable2D{
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

trait DebugMesurable2D: Mesurable2D + std::fmt::Debug {
    fn as_any(&self) -> &dyn Any;
}

impl<T: Mesurable2D + std::fmt::Debug + Any> DebugMesurable2D for T {
    fn as_any(&self) -> &dyn Any { self }
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
struct Polygon{
    // TODO
}

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


macro_rules! print_total_area {
    ($label:expr, $fn:expr, $source:expr) => {{
        let total_area = $fn($source);
        println!("{} total area: {}", $label, total_area.ceil());
    }};
    ($label:expr, $expr:expr) => {{
        let total_area: f64 = $expr;
        println!("{} total area: {}", $label, total_area.ceil());
    }};
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
    let mut mesurables: Vec<&dyn DebugMesurable2D> = Vec::new();
    circles.iter()
        .for_each(|c| mesurables.push(c));
     polygons.iter()
        .for_each(|p| mesurables.push(p));
        
    println!("{circles:#?}");
    println!("{polygons:#?}");
    println!("{mesurables:#?}");
    // pipeline direct
    print_total_area!("[direct vec of circle]", circles.iter().map(Circle::area).sum::<f64>());
    print_total_area!("[fun slice of circle]", total_area_slice_circle, &circles);
    print_total_area!("[fun slice of mesurable=circle]", total_area_slice_mesurable, &circles);
    print_total_area!("[fun iterator of mesurable=circle]", total_area_iterator_mesurable, circles.iter());
    print_total_area!("[fun iterator of mesurable=polygon]", total_area_iterator_mesurable, polygons.iter());
    // NB: attention le vecteur mesurables contient des references par rapport aux 2 précédents
    print_total_area!("[fun iterator of &dyn mesurable]", total_area_iterator_dyn_mesurable,
        mesurables.iter().copied().map(|m| m as &dyn Mesurable2D) // dereference + upcast
    );

    // les données sont toujours disponibles :)
    println!("Data still available:");
    mesurables.iter()
        .for_each(|m| {
            println!("\t- {m:?}");
            println!("\t\t* area: {}", m.area());
            println!("\t\t* perimeter: {}", m.perimeter());
            if let Some(circle) = m.as_any().downcast_ref::<Circle>() {
                let Point(x, y ) = circle.center;
                println!("\t\t* radius: {}", circle.radius);
                println!("\t\t* center: (x={x}, y={y})");
            } else if let Some(_polygon) = m.as_any().downcast_ref::<Polygon>() {
                println!("\t\t* polygon"); // TODO: use something specific about polygon
            }
        });
}

fn main() {
    play_with_mesurables();
}
