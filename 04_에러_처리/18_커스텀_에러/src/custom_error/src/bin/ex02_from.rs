// 18편 예제 2: #[from] 으로 From 자동 구현 + ? 자동 변환
//
// 외부 에러 타입(예: ParseIntError) 을 우리 enum 의 한 variant 로 감쌀 때
// `#[from]` 한 줄이면 From 트레잇이 자동으로 만들어집니다.

use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug, Error)]
enum MyError {
    #[error("정수 파싱 실패: {0}")]
    Parse(#[from] ParseIntError),     // ?  ParseIntError → MyError 자동
    #[error("값이 범위 밖: {0}")]
    OutOfRange(i32),
}

fn parse_positive(s: &str) -> Result<u32, MyError> {
    let n: i32 = s.parse()?;          // ParseIntError 가 자동으로 MyError 로
    if n < 0 {
        return Err(MyError::OutOfRange(n));
    }
    Ok(n as u32)
}

fn main() {
    for s in ["42", "abc", "-3"] {
        match parse_positive(s) {
            Ok(v) => println!("{s} → Ok({v})"),
            Err(e) => println!("{s} → Err: {e}"),
        }
    }
}
