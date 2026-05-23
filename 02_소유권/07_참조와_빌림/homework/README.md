# 과제 - 07. 참조와 빌림

## 문제 1 — 단어 개수 세기
- 파일명: `homework_01/src/main.rs`
- 핵심 개념: 공유 참조 `&str` / `&String`, `split_whitespace`

### 요구사항
- 함수 `word_count(s: &str) -> usize` 를 정의해 공백으로 나눈 단어 수를 반환한다.
- `main` 에서 `let text = String::from("hello rust borrowing checker");` 를 만들고
  함수에 참조를 넘긴다.
- 호출 후에도 `text` 를 다시 출력해 소유권이 이동하지 않았음을 보인다.

### 예상 출력
```
text = hello rust borrowing checker
단어 수 = 4
```

### 힌트
- `s.split_whitespace().count()`.
- `&text` 또는 `text.as_str()` 으로 `&str` 을 넘긴다.

## 문제 2 — Vec 모든 원소 두 배로
- 파일명: `homework_02/src/main.rs`
- 핵심 개념: 가변 참조 `&mut Vec<i32>`

### 요구사항
- 함수 `double_all(v: &mut Vec<i32>)` 를 정의해 모든 원소를 두 배로 만든다.
- `main` 에서 `let mut nums = vec![1, 2, 3, 4];` 를 만들어 호출 전후를 출력한다.

### 예상 출력
```
호출 전: [1, 2, 3, 4]
호출 후: [2, 4, 6, 8]
```

### 힌트
- `for n in v.iter_mut() { *n *= 2; }`
- `&mut nums` 로 가변 참조를 넘긴다.

## 정답 확인

직접 풀어 본 후 [`answer/`](./answer/) 폴더의 정답과 비교해 보세요.
