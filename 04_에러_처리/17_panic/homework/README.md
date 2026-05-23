# 과제 - 17. panic!

## 문제 1 — 안전한 인덱스 접근
- 파일명: `homework_01/src/main.rs`
- 핵심 개념: `Vec::get`, Option

### 요구사항
- 함수 `nth(v: &[i32], i: usize) -> String` 를 작성한다.
- `v.get(i)` 가 `Some(n)` 이면 `"v[{i}] = {n}"`, `None` 이면 `"v[{i}] 는 범위 밖"`.
- main 에서 `vec![10, 20, 30]` 에 대해 `i = 0, 2, 5` 호출 결과를 출력한다.

### 예상 출력
```
v[0] = 10
v[2] = 30
v[5] 는 범위 밖
```

### 힌트
- `match v.get(i) { Some(n) => format!(...), None => format!(...) }`.

## 문제 2 — 검증된 입력에 expect
- 파일명: `homework_02/src/main.rs`
- 핵심 개념: `expect("이유")` 의 활용

### 요구사항
- 배열 `let inputs = ["1", "2", "3"];` 가 주어진다.
- 모든 원소가 정수임을 우리는 알고 있다 — 그러므로 `expect("위에서 정수임을 검증")` 으로 파싱한다.
- 모두 합산해 출력한다.

### 예상 출력
```
합계 = 6
```

### 힌트
- `let sum: i32 = inputs.iter().map(|s| s.parse::<i32>().expect("...")).sum();`

## 정답 확인

직접 풀어 본 후 [`answer/`](./answer/) 폴더의 정답과 비교해 보세요.
