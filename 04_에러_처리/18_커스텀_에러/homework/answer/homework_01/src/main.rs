// 핵심 포인트:
// - struct-like variant 의 메시지는 `{필드이름}` 으로 직접 보간할 수 있다.
// - `chars().count()` 는 멀티바이트 문자도 정확히 셀 수 있는 안전한 방법.

use thiserror::Error;

#[derive(Debug, Error)]
enum UserError {
    #[error("이름이 비어 있습니다")]
    Empty,
    #[error("이름이 너무 짧습니다 (현재 {len}, 최소 {min})")]
    TooShort { len: usize, min: usize },
}

fn validate_name(name: &str) -> Result<String, UserError> {
    if name.is_empty() {
        return Err(UserError::Empty);
    }
    let len = name.chars().count();
    let min = 2;
    if len < min {
        return Err(UserError::TooShort { len, min });
    }
    Ok(name.to_string())
}

fn main() {
    for n in ["", "a", "Rust"] {
        match validate_name(n) {
            Ok(v) => println!("{n:?} → 유효: {v}"),
            Err(e) => println!("{n:?} → {e}"),
        }
    }
}
