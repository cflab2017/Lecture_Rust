// 20편: 라이브러리 코드 + 단위 테스트 + 문서 테스트(doctest)

/// 두 정수의 합을 돌려줍니다.
///
/// # 예시
/// ```
/// use testing::add;
/// assert_eq!(add(2, 3), 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// 정수가 짝수인지 검사합니다.
///
/// # 예시
/// ```
/// use testing::is_even;
/// assert!(is_even(4));
/// assert!(!is_even(7));
/// ```
pub fn is_even(n: i32) -> bool {
    n % 2 == 0
}

// 같은 파일 안에 단위 테스트 모듈을 두는 것이 Rust 의 관례입니다.
// `#[cfg(test)]` 덕에 `cargo test` 때만 컴파일됩니다.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_positive() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn add_zero() {
        assert_eq!(add(0, 0), 0);
    }

    #[test]
    fn even_true() {
        assert!(is_even(4));
        assert!(is_even(0));
    }

    #[test]
    fn even_false() {
        assert!(!is_even(7));
    }
}
