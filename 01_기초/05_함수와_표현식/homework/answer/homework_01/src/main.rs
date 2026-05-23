// 핵심 포인트:
// - `if a > b { a } else { b }` 는 표현식이라 그 자체로 반환값이 된다.
// - 마지막 줄에 세미콜론을 붙이면 `()` 가 반환되어 타입 에러가 난다.

fn max_of(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

fn main() {
    println!("max(3, 7) = {}", max_of(3, 7));
    println!("max(10, 4) = {}", max_of(10, 4));
    println!("max(-2, -5) = {}", max_of(-2, -5));
}
