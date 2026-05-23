# 과제 - 20. 테스트

## 문제 1 — `abs` 와 단위 테스트
- 파일명: `homework_01/src/main.rs`
- 핵심 개념: `#[test]`, `assert_eq!`

### 요구사항
- 함수 `abs(n: i32) -> i32` 를 정의한다(절댓값).
- `#[cfg(test)] mod tests` 안에 다음 4 개 테스트를 작성한다.
  - `abs(-7) == 7`
  - `abs(7) == 7`
  - `abs(0) == 0`
  - `abs(i32::MIN.checked_neg().unwrap_or(0))` 같은 극단 케이스는 생략 가능 — 단순 양수·음수·0 만 다루면 됩니다.
- main 에서 `abs(-9)` 를 출력한다.

### 예상 출력 (cargo run)
```
abs(-9) = 9
```

### 예상 결과 (cargo test)
- 모든 테스트가 PASS.

### 힌트
- `if n < 0 { -n } else { n }`.
- 테스트 모듈: `#[cfg(test)] mod tests { use super::*; #[test] fn ... {} }`.

## 문제 2 — `is_prime` 와 단위 테스트
- 파일명: `homework_02/src/main.rs`
- 핵심 개념: `#[test]`, `assert!`/`assert!(!...)`

### 요구사항
- 함수 `is_prime(n: u32) -> bool` 를 정의한다.
  - 0, 1 은 false.
  - 2 부터 시작해 √n 까지 나누어 본다.
- `#[cfg(test)] mod tests` 안에 다음 테스트들을 둔다.
  - `is_prime(0) == false`, `is_prime(1) == false`
  - `is_prime(2)`, `is_prime(3)`, `is_prime(7)`, `is_prime(13)` 모두 true
  - `is_prime(4) == false`, `is_prime(9) == false`
- main 에서 7 의 소수 여부를 출력한다.

### 예상 출력 (cargo run)
```
is_prime(7) = true
```

### 예상 결과 (cargo test)
- 모든 테스트가 PASS.

### 힌트
- `if n < 2 { return false; }`.
- `let mut i = 2; while i * i <= n { if n % i == 0 { return false; } i += 1; } true`.

## 정답 확인

직접 풀어 본 후 [`answer/`](./answer/) 폴더의 정답과 비교해 보세요.
정답은 `cargo run` 으로 main 결과를, `cargo test` 로 테스트 결과를 확인할 수 있습니다.
