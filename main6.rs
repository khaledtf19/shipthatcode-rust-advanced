// TODO: trait Shape with one method: fn area(&self) -> f64
// TODO: struct Square { side: f64 }
// TODO: struct Triangle { base: f64, height: f64 }
// TODO: impl Shape for each of them

trait Shape {
    fn area(&self) -> f64;
}
struct Square {
    side: f64,
}
struct Triangle {
    base: f64,
    height: f64,
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        (self.base * self.height) / 2.0
    }
}
impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}
fn main() {
    // TODO: build a Vec<Box<dyn Shape>> with the square then the triangle,
    //       then print each area with {:.2} on its own line.
    let v: Vec<Box<dyn Shape>> = vec![
        Box::new(Square { side: 3.0 }),
        Box::new(Triangle {
            base: 4.0,
            height: 5.0,
        }),
    ];

    for shape in v {
        println!("{}", shape.area());
    }
}
