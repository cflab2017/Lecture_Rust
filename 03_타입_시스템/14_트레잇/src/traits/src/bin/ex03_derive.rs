// 14편 예제 3: derive 매크로로 흔한 트레잇 자동 구현
//
// 자주 쓰는 트레잇은 `#[derive(...)]` 로 자동 구현 가능합니다.

#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 1, y: 2 };

    // Clone — 깊은 복사본
    let p2 = p1.clone();

    // PartialEq — == / != 비교 가능
    println!("p1 == p2 ? {}", p1 == p2);

    // Debug — {:?} / {:#?} 로 출력 가능
    println!("p1 = {:?}", p1);
    println!("p1 = {:#?}", p1); // 들여쓰기 포맷
}
