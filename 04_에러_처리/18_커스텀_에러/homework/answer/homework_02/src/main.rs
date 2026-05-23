// 핵심 포인트:
// - `#[from] ParseIntError` 한 줄로 From 구현이 자동 — `?` 가 변환을 알아서 해 준다.
// - 호출자는 SumError 하나만 알면 두 단계 파싱 실패를 모두 잡을 수 있다.

use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug, Error)]
enum SumError {
    #[error("파싱 실패: {0}")]
    Parse(#[from] ParseIntError),
}

fn sum_strings(a: &str, b: &str) -> Result<i32, SumError> {
    let x: i32 = a.parse()?;
    let y: i32 = b.parse()?;
    Ok(x + y)
}

fn main() {
    let cases = [("3", "4"), ("12", "x")];
    for (a, b) in cases {
        let label = format!("({:?}, {:?})", a, b);
        match sum_strings(a, b) {
            Ok(n) => println!("{label} → Ok({n})"),
            Err(e) => println!("{label} → Err: {e}"),
        }
    }
}
