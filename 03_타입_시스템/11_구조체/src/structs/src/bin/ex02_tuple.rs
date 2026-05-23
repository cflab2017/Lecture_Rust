// 11편 예제 2: 튜플 구조체 — 이름은 있지만 필드 이름은 없음

#[derive(Debug)]
struct Color(u8, u8, u8);

#[derive(Debug)]
struct Point(f64, f64);

impl Color {
    fn brightness(&self) -> u16 {
        // u8 끼리 더하면 오버플로 위험 → u16 으로 확장
        self.0 as u16 + self.1 as u16 + self.2 as u16
    }
}

fn main() {
    let red = Color(255, 0, 0);
    let origin = Point(0.0, 0.0);

    println!("red = {:?}, brightness = {}", red, red.brightness());
    println!("origin = ({}, {})", origin.0, origin.1);

    // 튜플 구조체끼리는 타입이 다르면 서로 호환되지 않음
    // let x: Color = origin; // ❌
}
