# 과제 - 09. 라이프타임 입문

## 문제 1 — `shortest` 함수
- 파일명: `homework_01/src/main.rs`
- 핵심 개념: `<'a>` 라이프타임 명시

### 요구사항
- 함수 `shortest<'a>(a: &'a str, b: &'a str) -> &'a str` 를 작성한다.
- 두 슬라이스 중 짧은 쪽(같으면 a)을 돌려준다.
- main 에서 `"hello"` 와 `"hi"`, `"같음"` 과 `"같음"` 으로 호출해 결과를 출력한다.

### 예상 출력
```
짧은 것: hi
짧은 것: 같음
```

### 힌트
- `if a.len() <= b.len() { a } else { b }`.

## 문제 2 — `Note` 구조체
- 파일명: `homework_02/src/main.rs`
- 핵심 개념: 참조 필드를 가진 구조체, 라이프타임 파라미터

### 요구사항
- `Note<'a>` 구조체에 `title: &'a str`, `body: &'a str` 두 필드를 둔다.
- 메서드 `summary(&self) -> String` 를 정의해 `"{title}: {앞 20글자}…"` 형식으로 돌려준다.
  - body 가 20글자 이하면 그대로, 넘으면 앞 20글자 + `…`.
- main 에서 두 가지 Note 를 만들어 출력한다.

### 예상 출력
```
오늘의 메모: 짧은 본문.
긴 메모: This is a longer bod…
```

### 힌트
- `body.chars().count()` 와 `body.chars().take(20).collect::<String>()`.

## 정답 확인

직접 풀어 본 후 [`answer/`](./answer/) 폴더의 정답과 비교해 보세요.
