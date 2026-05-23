# 20. 단위 테스트·통합 테스트·doctest

테스트는 Rust 가 가장 자랑하는 기능 중 하나입니다. 컴파일러가 코드를 강하게 검사한다고는 해도 비즈니스 로직의 정합성은 컴파일러가 알 수 없죠. Rust 는 단위 테스트·통합 테스트·문서 테스트 세 종류를 표준으로 지원하고, 모두 `cargo test` 한 줄에 실행됩니다.

## 학습 목표

- `#[cfg(test)] mod tests` 안의 `#[test]` 함수로 단위 테스트를 작성한다.
- `tests/` 디렉터리에 통합 테스트를 두고 외부 사용자처럼 라이브러리를 검증한다.
- `///` 문서화 주석의 코드 블록이 doctest 로 실행됨을 이해한다.
- `assert!`, `assert_eq!`, `assert_ne!`, `should_panic` 등 자주 쓰는 매크로를 안다.

## 핵심 개념

### 1) 단위 테스트 — 같은 파일 안에

```rust
pub fn add(a: i32, b: i32) -> i32 { a + b }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_positive() {
        assert_eq!(add(2, 3), 5);
    }
}
```

- `#[cfg(test)]` 가 붙은 `mod tests` 는 `cargo test` 때만 컴파일됩니다.
- `use super::*;` 로 같은 파일(부모 모듈)의 비공개 항목까지 접근 가능 — 단위 테스트의 강점.

### 2) 통합 테스트 — `tests/` 디렉터리

```
hello/
├── Cargo.toml
├── src/
│   └── lib.rs
└── tests/
    └── integration_test.rs
```

`tests/` 안의 각 `.rs` 파일은 독립된 크레잇처럼 컴파일됩니다. 라이브러리의 **공개 API 만** 사용 가능하므로 외부 사용자의 시선으로 검증할 수 있습니다.

### 3) doctest — `///` 코드 블록

```rust
/// # 예시
/// ```
/// use mycrate::add;
/// assert_eq!(add(2, 3), 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 { a + b }
```

문서 주석의 ```` ``` ```` 안 코드는 doctest 로 자동 실행됩니다. 문서와 테스트가 동시에 만들어지죠. (단, 라이브러리 크레잇의 공개 API 에서만 동작합니다.)

### 4) 자주 쓰는 매크로

| 매크로 | 의미 |
|--------|------|
| `assert!(expr)` | true 가 아니면 패닉 |
| `assert_eq!(a, b)` | a == b 가 아니면 패닉 (양쪽 값 출력) |
| `assert_ne!(a, b)` | a != b 가 아니면 패닉 |
| `#[should_panic]` | 본문이 패닉해야 통과 |
| `#[ignore]` | 평소엔 건너뛰고 `cargo test -- --ignored` 로만 실행 |

### 5) cargo test 명령

```sh
cargo test                          # 모든 테스트
cargo test add_positive             # 이름에 add_positive 가 포함된 것만
cargo test --lib                    # 단위 테스트만
cargo test --test integration_test  # tests/ 안의 특정 파일만
cargo test --doc                    # doctest 만
cargo test -- --nocapture           # 테스트 안 println! 까지 출력
```

## 예제로 보기

### 예제 1 — `src/lib.rs` : 단위 테스트와 doctest

```rust
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
```

### 예제 2 — `tests/integration_test.rs` : 통합 테스트

```rust
// `tests/` 폴더의 파일은 통합 테스트(integration test) 로 취급됩니다.
// 외부 사용자처럼 라이브러리의 공개 API 만 사용해 검증합니다.

use testing::{add, is_even};

#[test]
fn integration_add() {
    assert_eq!(add(10, 20), 30);
    assert_eq!(add(-5, 5), 0);
}

#[test]
fn integration_even() {
    for n in [0, 2, 4, 100] {
        assert!(is_even(n));
    }
    for n in [1, 3, 5] {
        assert!(!is_even(n));
    }
}
```

### 예제 3 — `src/main.rs` : 라이브러리 사용

```rust
// 라이브러리(`testing`) 의 API 를 사용하는 작은 실행 파일.
// 테스트는 `cargo test` 로 실행하세요.

use testing::{add, is_even};

fn main() {
    println!("add(2, 3) = {}", add(2, 3));
    println!("is_even(4) = {}", is_even(4));

    println!();
    println!("이 크레잇의 테스트는 다음 명령으로 실행하세요:");
    println!("  cargo test                          # 전체");
    println!("  cargo test --lib                    # 단위 테스트만");
    println!("  cargo test --test integration_test  # 통합 테스트만");
    println!("  cargo test --doc                    # doctest 만");
}
```

## 자주 하는 실수

### Q. `cargo test` 가 아무것도 찾지 못합니다.

A. 테스트 함수에 `#[test]` 가 빠졌거나, `#[cfg(test)] mod tests` 외부에 함수가 있는지 확인하세요. 통합 테스트는 `tests/` 디렉터리 안에 있어야 합니다.

### Q. doctest 가 안 돌아요.

A. doctest 는 **라이브러리 크레잇의 공개 API** 에서만 동작합니다. 바이너리 전용(`src/main.rs` 만 있는) 크레잇에서는 doctest 가 무시됩니다. 이번 예제처럼 `src/lib.rs` 가 있어야 합니다.

### Q. `assert_eq!` 가 실패할 때 어떻게 메시지를 더 풍부하게 줄 수 있나요?

A. 추가 인수를 받습니다.
```rust
assert_eq!(add(2, 3), 5, "두 양수의 합");
```

### Q. 패닉이 나는 게 정상인 함수를 테스트하려면?

A. `#[should_panic]` 또는 `#[should_panic(expected = "메시지의 일부")]` 어트리뷰트를 붙이세요. 본문이 패닉하지 않으면 테스트 실패로 처리됩니다.

## 정리

- 단위 테스트는 같은 파일 안 `#[cfg(test)] mod tests` — 비공개 항목까지 접근 가능.
- 통합 테스트는 `tests/` 디렉터리 — 공개 API 만 사용, 외부 시각으로 검증.
- 문서 테스트는 `///` 의 ```` ``` ```` 블록 — 문서와 검증을 한 곳에서.
- 모든 테스트는 `cargo test` 한 줄로 실행 — 필터·플래그로 부분 실행 가능.

## 직접 해 보기

`homework/` 폴더의 과제를 풀어 보세요. 정답은 `homework/answer/` 에 있습니다.

## 다음 단원

[21. 스레드·채널·Arc/Mutex](../21_동시성/README.md) — 표준 라이브러리의 동시성 기본기를 배웁니다.
