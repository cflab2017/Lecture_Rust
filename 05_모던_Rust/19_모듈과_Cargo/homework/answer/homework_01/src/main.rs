// 핵심 포인트:
// - `pub` 가 없으면 모듈 밖에서 함수를 부를 수 없다.
// - `use 모듈::항목` 으로 가져오면 경로를 줄여 가독성이 좋아진다.

mod math {
    pub fn add(a: i32, b: i32) -> i32 { a + b }
    pub fn mul(a: i32, b: i32) -> i32 { a * b }
}

use math::{add, mul};

fn main() {
    println!("add(3, 4) = {}", add(3, 4));
    println!("mul(3, 4) = {}", mul(3, 4));
}
