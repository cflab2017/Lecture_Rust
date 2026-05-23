// 핵심 포인트:
// - enum 의 각 variant 가 다른 모양의 데이터를 가져도 한 타입으로 처리할 수 있다.
// - match 로 variant 를 분해하면서 동시에 필드 값을 바인딩한다.

use std::f64::consts::PI;

enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64),
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(r) => PI * r * r,
            Shape::Rectangle(w, h) => w * h,
            Shape::Triangle(b, h) => 0.5 * b * h,
        }
    }
}

fn main() {
    let shapes = [
        Shape::Circle(5.0),
        Shape::Rectangle(4.0, 3.0),
        Shape::Triangle(6.0, 4.0),
    ];

    let labels = ["Circle(5.0)        ", "Rectangle(4.0, 3.0)", "Triangle(6.0, 4.0) "];
    for (label, shape) in labels.iter().zip(shapes.iter()) {
        println!("{label} 의 넓이 = {:.2}", shape.area());
    }
}
