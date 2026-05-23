// 핵심 포인트:
// - `Self::new` 같은 연관 함수는 흔한 생성자 패턴이다.
// - `std::f64::consts::PI` 는 표준 라이브러리 상수로 정확도가 높다.

use std::f64::consts::PI;

struct Circle {
    radius: f64,
}

impl Circle {
    fn new(r: f64) -> Self {
        Self { radius: r }
    }

    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }

    fn circumference(&self) -> f64 {
        2.0 * PI * self.radius
    }
}

fn main() {
    let c = Circle::new(5.0);
    println!("넓이: {:.2}", c.area());
    println!("둘레: {:.2}", c.circumference());
}
