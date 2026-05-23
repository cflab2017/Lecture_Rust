// 핵심 포인트:
// - 함수 반환 타입이 Result<_, ParseIntError> 라 ? 가 두 번 모두 같은 에러로 전파된다.
// - 호출 측에서는 결과를 한 번에 받아 분기한다.

use std::num::ParseIntError;

fn add_strings(a: &str, b: &str) -> Result<i32, ParseIntError> {
    let x: i32 = a.parse()?;
    let y: i32 = b.parse()?;
    Ok(x + y)
}

fn main() {
    let cases = [("3", "4"), ("12", "x")];
    for (a, b) in cases {
        let label = format!("({:?}, {:?})", a, b);
        match add_strings(a, b) {
            Ok(n) => println!("{label} → Ok({n})"),
            Err(e) => println!("{label} → Err({e:?})"),
        }
    }
}
