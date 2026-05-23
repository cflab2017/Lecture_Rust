# 과제 - 12. 열거형과 Option

## 문제 1 — Shape enum 의 넓이
- 파일명: `homework_01/src/main.rs`
- 핵심 개념: enum + 데이터, `match`, 메서드

### 요구사항
- `Shape` enum 을 정의한다.
  - `Circle(f64)` — 반지름
  - `Rectangle(f64, f64)` — 너비, 높이
  - `Triangle(f64, f64)` — 밑변, 높이
- 메서드 `area(&self) -> f64` 가 각 도형의 넓이를 돌려준다.
- main 에서 세 도형의 넓이를 출력한다.

### 예상 출력
```
Circle(5.0)         의 넓이 = 78.54
Rectangle(4.0, 3.0) 의 넓이 = 12.00
Triangle(6.0, 4.0)  의 넓이 = 12.00
```

### 힌트
- `std::f64::consts::PI`.
- 삼각형 넓이 = 0.5 * 밑변 * 높이.
- `match self { Shape::Circle(r) => ..., ... }`.

## 문제 2 — `safe_div` (0 나눗셈 대응)
- 파일명: `homework_02/src/main.rs`
- 핵심 개념: `Option<f64>`, 패턴 처리

### 요구사항
- 함수 `safe_div(a: f64, b: f64) -> Option<f64>` 를 정의한다.
- `b == 0.0` 이면 `None`, 그 외에는 `Some(a / b)`.
- main 에서 `(10.0, 2.0)`, `(7.0, 0.0)` 두 호출의 결과를 다음처럼 처리한다.
  - Some: `결과: 5.00`
  - None: `0으로 나눌 수 없음`

### 예상 출력
```
결과: 5.00
0으로 나눌 수 없음
```

### 힌트
- `if b == 0.0 { None } else { Some(a / b) }`.
- `match result { Some(v) => println!("결과: {:.2}", v), None => println!("0으로 나눌 수 없음") }`.

## 정답 확인

직접 풀어 본 후 [`answer/`](./answer/) 폴더의 정답과 비교해 보세요.
