# 과제 - 10. String 과 &str

## 문제 1 — 문자열 뒤집기
- 파일명: `homework_01/src/main.rs`
- 핵심 개념: `chars()`, `rev()`, `collect`

### 요구사항
- 함수 `reverse(s: &str) -> String` 를 작성한다.
- `"Hello"` 와 `"Rust 한글"` 두 입력에 대해 결과를 출력한다.

### 예상 출력
```
reverse("Hello") = olleH
reverse("Rust 한글") = 글한 tsuR
```

### 힌트
- `s.chars().rev().collect()`.
- 반환이 `String` 이므로 `collect::<String>()` 또는 좌변 타입 명시 필요.

## 문제 2 — 모음 개수 세기
- 파일명: `homework_02/src/main.rs`
- 핵심 개념: `chars()` + 필터, ASCII 모음 판별

### 요구사항
- 함수 `count_vowels(s: &str) -> usize` 를 작성한다.
- 대소문자 a/e/i/o/u 를 모두 모음으로 센다.
- main 에서 `"Programming in Rust"` 입력에 대해 결과 출력.

### 예상 출력
```
\"Programming in Rust\" 의 모음 개수 = 5
```

### 힌트
- `c.to_ascii_lowercase()` 후 `matches!(c, 'a'|'e'|'i'|'o'|'u')`.
- `filter().count()`.

## 정답 확인

직접 풀어 본 후 [`answer/`](./answer/) 폴더의 정답과 비교해 보세요.
