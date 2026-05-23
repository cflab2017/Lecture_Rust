// 18편 예제 1: thiserror 로 커스텀 에러 enum 정의
//
// `#[derive(Error)]` 와 `#[error("...")]` 만으로 Display + Error 트레잇이 자동 구현됩니다.

use thiserror::Error;

#[derive(Debug, Error)]
enum AgeError {
    #[error("나이는 0 이상이어야 합니다 (현재 {0})")]
    Negative(i32),
    #[error("나이가 너무 큽니다 (최대 150, 현재 {0})")]
    TooLarge(i32),
}

fn validate_age(n: i32) -> Result<u8, AgeError> {
    if n < 0 {
        return Err(AgeError::Negative(n));
    }
    if n > 150 {
        return Err(AgeError::TooLarge(n));
    }
    Ok(n as u8)
}

fn main() {
    for n in [25, -3, 200] {
        match validate_age(n) {
            Ok(v) => println!("{n} → 유효: {v}"),
            // {} 는 Display, {:?} 는 Debug — thiserror 가 둘 다 깔끔하게 만들어 줌
            Err(e) => println!("{n} → 에러: {e}"),
        }
    }
}
