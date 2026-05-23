// 14편 예제 2: trait object (`dyn Shape`) — 런타임 다형성
//
// 트레잇을 구현한 서로 다른 타입을 같은 컬렉션에 넣고 동일 메서드를 호출하려면
// "trait object" 를 사용합니다. 보통 `Box<dyn Trait>` 또는 `&dyn Trait`.

use std::f64::consts::PI;

trait Shape {
    fn area(&self) -> f64;
    fn name(&self) -> &'static str;
}

struct Circle { r: f64 }
struct Square { side: f64 }

impl Shape for Circle {
    fn area(&self) -> f64 { PI * self.r * self.r }
    fn name(&self) -> &'static str { "Circle" }
}

impl Shape for Square {
    fn area(&self) -> f64 { self.side * self.side }
    fn name(&self) -> &'static str { "Square" }
}

fn print_area(s: &dyn Shape) {
    println!("{}: 넓이 = {:.2}", s.name(), s.area());
}

fn main() {
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { r: 3.0 }),
        Box::new(Square { side: 4.0 }),
    ];
    for s in &shapes {
        print_area(s.as_ref());
    }
}
