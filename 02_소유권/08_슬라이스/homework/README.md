# 과제 - 08. 슬라이스

## 문제 1 — `nth_word` (n 번째 단어)
- 파일명: `homework_01/src/main.rs`
- 핵심 개념: `&str`, `split_whitespace`, `nth`

### 요구사항
- 함수 `nth_word(s: &str, n: usize) -> Option<&str>` 를 작성한다.
- 0 부터 시작해 `n` 번째 공백 단어를 돌려준다. 없으면 `None`.
- main 에서 다음 입력으로 결과를 출력한다.
  - 입력: `"Rust is a systems language"`, n = 0, 1, 2, 10

### 예상 출력
```
0번째 단어: Some("Rust")
1번째 단어: Some("is")
2번째 단어: Some("a")
10번째 단어: None
```

### 힌트
- `s.split_whitespace().nth(n)`.
- 반환 타입에 라이프타임이 자동 추론된다 (9편 참고).

## 문제 2 — 슬라이스 평균
- 파일명: `homework_02/src/main.rs`
- 핵심 개념: `&[f64]`, 평균 계산

### 요구사항
- 함수 `average(values: &[f64]) -> f64` 를 작성한다 (빈 슬라이스면 0.0 반환).
- main 에서 다음을 확인한다.
  - 배열 `[10.0, 20.0, 30.0, 40.0]` 의 평균
  - 빈 슬라이스 `&[]` 의 평균

### 예상 출력
```
[10.0, 20.0, 30.0, 40.0] 의 평균 = 25.00
빈 슬라이스의 평균 = 0.00
```

### 힌트
- `values.is_empty()` 로 분기.
- `values.iter().sum::<f64>() / values.len() as f64`.

## 정답 확인

직접 풀어 본 후 [`answer/`](./answer/) 폴더의 정답과 비교해 보세요.
