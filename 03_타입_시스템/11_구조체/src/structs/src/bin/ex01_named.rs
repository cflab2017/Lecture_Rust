// 11편 예제 1: named-field 구조체와 메서드(impl)
//
// 구조체는 도메인 개념을 타입으로 표현하는 가장 흔한 도구입니다.

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 연관 함수(Associated function) — Self::new 같은 생성자
    fn new(w: u32, h: u32) -> Self {
        Self { width: w, height: h }
    }

    // 메서드 — 첫 인수가 &self
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 다른 사각형과 비교하는 메서드
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let r1 = Rectangle::new(30, 20);
    let r2 = Rectangle { width: 10, height: 5 }; // 필드 직접 초기화도 가능

    println!("{:?} 의 넓이 = {}", r1, r1.area());
    println!("{:?} 는 {:?} 를 포함? {}", r1, r2, r1.can_hold(&r2));
}
