enum Shape {
    Circle(f64),        // promień
    Rectangle(f64, f64), // szerokość, wysokość
    Square(f64),        // bok
}

impl Shape {
    fn area(&self) -> f64 {
        match *self {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Rectangle(width, height) => width * height,
            Shape::Square(side) => side * side,
        }
    }
}

pub fn test() {
    let shapes = vec![
        Shape::Circle(2.0),
        Shape::Rectangle(3.0, 4.0),
        Shape::Square(5.0),
    ];

    for shape in shapes {
        println!("Area: {}", shape.area());
    }
}