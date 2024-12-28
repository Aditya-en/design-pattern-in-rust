trait Shape {
    fn area(&self) -> f64;
}

#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}
impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

#[derive(Debug)]
struct Circle {
    radius: f64,
}
impl Shape for Circle {
    fn area(&self) -> f64 {
        3.14159 * self.radius * self.radius
    }
}

#[derive(Debug)]
struct EqTriangle {
    size: f64,
}
impl Shape for EqTriangle {
    fn area(&self) -> f64 {
        self.size * self.size * f64::sqrt(3.0) / 4.0
    }
}

#[derive(Debug)]
struct Square {
    side: f64,
}
impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

enum ShapeType {
    Circle,
    Rectangle,
    Square,
    EqTriangle,
}

fn factory(shape_type: ShapeType, dim: Vec<f64>) -> Box<dyn Shape>{
    match shape_type {
        ShapeType::Circle => {
            if dim.len() < 1 {
                panic!("The dim should have at least 1 element");
            }
            Box::new(Circle {radius: dim[0]})
        }
        ShapeType::Rectangle => {
            if dim.len() < 2 {
                panic!("The dim should have at least 2 elements for a rectangle");
            }
            Box::new(Rectangle {width: dim[0], height:dim[1]})
        }
        ShapeType::EqTriangle => {
            if dim.len() < 1 {
                panic!("The dim should have at least 1 element");

            }
            Box::new(EqTriangle {size: dim[0]})
        }
        ShapeType::Square => {
            if dim.len() < 1 {
                panic!("The dim should have at least 1 elements");

            }
            Box::new(Square {side: dim[0]})
        }
    }
}

fn main() {
    let side = 10.8 as f64;
    let radius: f64 = 16.2;
    let size: f64 = 7.1;
    let sq = factory(ShapeType::Square, vec![side]);
    let tri = factory(ShapeType::EqTriangle, vec![size]);
    let circle = factory(ShapeType::Circle, vec![radius]);
    let rect = factory(ShapeType::Rectangle, vec![12.2 as f64, 18.8 as f64]);
    println!("Square is {:?} ", sq.area());
    println!("The rect {:?} ", rect.area());
    println!("Circle is {:?} ", circle.area());
    println!("triangle is {:?}", tri.area());
}
