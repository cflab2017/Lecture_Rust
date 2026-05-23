// 핵심 포인트:
// - 단위 테스트는 함수 정의와 같은 파일에 두는 것이 일반적이다.
// - main.rs 안에도 `#[cfg(test)] mod tests` 를 둘 수 있고, `cargo test` 가 자동으로 인식한다.

fn abs(n: i32) -> i32 {
    if n < 0 { -n } else { n }
}

fn main() {
    println!("abs(-9) = {}", abs(-9));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn negative() {
        assert_eq!(abs(-7), 7);
    }

    #[test]
    fn positive() {
        assert_eq!(abs(7), 7);
    }

    #[test]
    fn zero() {
        assert_eq!(abs(0), 0);
    }
}
