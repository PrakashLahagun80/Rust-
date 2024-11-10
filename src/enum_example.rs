pub enum Shape {
    Rectangle(f64, f64),
    Circle(f64)
}

pub fn calculate_area(shape:Shape)->f64 {
    let area = match shape {
        Shape::Rectangle(a,b)=>a*b,
        Shape::Circle(r)=>3.14*r*r
    };
    return area;
}