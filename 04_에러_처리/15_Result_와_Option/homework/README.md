# 과제 - 15. Result 와 Option 다루기

## 문제 1 — 안전한 parse
- 파일명: `homework_01/src/main.rs`
- 핵심 개념: `Result`, `unwrap_or`, `map`

### 요구사항
- 함수 `parse_or_default(s: &str) -> i32` 를 정의한다.
- 입력을 i32 로 파싱하고, 실패하면 0 을 돌려준다.
- main 에서 `"42"`, `"abc"`, `"  -7  "` 세 입력에 대한 결과를 출력한다.

### 예상 출력
```
42
0
-7
```

### 힌트
- `s.trim().parse::<i32>().unwrap_or(0)`.

## 문제 2 — Option 체이닝
- 파일명: `homework_02/src/main.rs`
- 핵심 개념: `and_then`, Option 반환 함수 합성

### 요구사항
- 함수 `half(n: i32) -> Option<i32>` 는 짝수면 `Some(n/2)`, 홀수면 `None`.
- 함수 `triple(n: i32) -> Option<i32>` 는 항상 `Some(n*3)` (간단히).
- main 에서 다음 두 입력을 처리한다.
  - 입력 16 → half → triple → half 체이닝
  - 입력 9  → half → triple → half (첫 step 에서 None)

### 예상 출력
```
16 → Some(12)
9 → None
```

### 힌트
- `Some(16).and_then(half).and_then(triple).and_then(half)`.

## 정답 확인

직접 풀어 본 후 [`answer/`](./answer/) 폴더의 정답과 비교해 보세요.
