# 과제 - 18. 커스텀 에러 타입

## 문제 1 — `UserError` 정의
- 파일명: `homework_01/src/main.rs`
- 핵심 개념: `thiserror::Error`, `#[error]`

### 요구사항
- `UserError` enum 을 두 variant 로 정의한다.
  - `Empty` — 메시지: `"이름이 비어 있습니다"`
  - `TooShort { len: usize, min: usize }` — 메시지: `"이름이 너무 짧습니다 (현재 {len}, 최소 {min})"`
- 함수 `validate_name(name: &str) -> Result<String, UserError>` 가
  - 빈 문자열이면 `Err(UserError::Empty)`
  - 2 문자 미만이면 `Err(UserError::TooShort { len, min: 2 })`
  - 그 외에는 `Ok(name.to_string())`
- main 에서 `""`, `"a"`, `"Rust"` 각각의 결과를 출력한다.

### 예상 출력
```
"" → 이름이 비어 있습니다
"a" → 이름이 너무 짧습니다 (현재 1, 최소 2)
"Rust" → 유효: Rust
```

### 힌트
- `name.chars().count()` 로 문자 길이.
- struct-like variant 메시지: `#[error("이름이 너무 짧습니다 (현재 {len}, 최소 {min})")]`.

## 문제 2 — 두 입력 합산 + 외부 에러 통합
- 파일명: `homework_02/src/main.rs`
- 핵심 개념: `#[from]`, `?` 자동 변환

### 요구사항
- `SumError` enum 의 한 variant 가 `Parse(#[from] std::num::ParseIntError)` 가 되도록 정의한다.
  - 메시지: `"파싱 실패: {0}"`
- 함수 `sum_strings(a: &str, b: &str) -> Result<i32, SumError>` 가 두 입력의 합을 돌려준다 (parse 실패는 자동 전파).
- main 에서 `("3", "4")`, `("12", "x")` 두 호출의 결과를 출력한다.

### 예상 출력
```
("3", "4") → Ok(7)
("12", "x") → Err: 파싱 실패: invalid digit found in string
```

### 힌트
- `#[derive(Debug, Error)]`.
- `let x: i32 = a.parse()?;` 처럼 ? 사용.

## 정답 확인

직접 풀어 본 후 [`answer/`](./answer/) 폴더의 정답과 비교해 보세요.
