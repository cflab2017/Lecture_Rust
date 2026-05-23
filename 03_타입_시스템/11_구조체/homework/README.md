# 과제 - 11. 구조체

## 문제 1 — Circle 의 넓이
- 파일명: `homework_01/src/main.rs`
- 핵심 개념: named-field struct, `impl`, 연관 함수

### 요구사항
- `Circle { radius: f64 }` 구조체를 정의한다.
- 연관 함수 `Circle::new(r: f64) -> Self` 를 정의한다.
- 메서드 `area(&self) -> f64`, `circumference(&self) -> f64` 를 정의한다.
- 원주율은 `std::f64::consts::PI` 사용.
- main 에서 반지름 5.0 짜리 원을 만들어 넓이·둘레를 출력한다.

### 예상 출력
```
넓이: 78.54
둘레: 31.42
```

### 힌트
- `area = π * r²`, `circumference = 2 * π * r`.
- `f64` 거듭제곱: `r.powi(2)` 또는 `r * r`.
- 포맷: `{:.2}`.

## 문제 2 — `Rgb` 튜플 구조체와 색 반전
- 파일명: `homework_02/src/main.rs`
- 핵심 개념: tuple struct, `impl`

### 요구사항
- `struct Rgb(u8, u8, u8);` 를 정의한다.
- 메서드 `invert(&self) -> Rgb` 가 `(255-r, 255-g, 255-b)` 인 새 색을 돌려준다.
- main 에서 빨강(255,0,0)을 반전한 결과를 출력한다.

### 예상 출력
```
원본: Rgb(255, 0, 0)
반전: Rgb(0, 255, 255)
```

### 힌트
- `#[derive(Debug)]`.
- 새 인스턴스: `Rgb(255 - self.0, 255 - self.1, 255 - self.2)`.

## 정답 확인

직접 풀어 본 후 [`answer/`](./answer/) 폴더의 정답과 비교해 보세요.
