# 과제 - 19. 모듈과 Cargo

## 문제 1 — `math` 모듈
- 파일명: `homework_01/src/main.rs`
- 핵심 개념: 인라인 모듈, `pub`, `use`

### 요구사항
- 같은 파일 안에 `math` 모듈을 정의한다.
  - `pub fn add(a: i32, b: i32) -> i32`
  - `pub fn mul(a: i32, b: i32) -> i32`
- `use math::{add, mul};` 으로 짧은 이름을 사용한다.
- main 에서 다음을 출력한다.
  - `add(3, 4) = 7`
  - `mul(3, 4) = 12`

### 예상 출력
```
add(3, 4) = 7
mul(3, 4) = 12
```

### 힌트
- `mod math { pub fn add(...) { ... } pub fn mul(...) { ... } }`.

## 문제 2 — `shape::circle` 중첩 모듈
- 파일명: `homework_02/src/main.rs`
- 핵심 개념: 중첩 모듈, 경로 접근

### 요구사항
- `shape` 모듈 안에 `circle` 자식 모듈을 둔다.
- `shape::circle::area(r: f64) -> f64` 가 원의 넓이를 돌려준다 (π * r²).
- main 에서 r = 4.0 으로 호출해 결과를 출력한다.

### 예상 출력
```
circle area (r=4) = 50.27
```

### 힌트
- `std::f64::consts::PI`.
- `mod shape { pub mod circle { ... } }`.

## 정답 확인

직접 풀어 본 후 [`answer/`](./answer/) 폴더의 정답과 비교해 보세요.
