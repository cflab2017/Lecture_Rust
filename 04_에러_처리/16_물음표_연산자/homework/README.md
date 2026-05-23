# 과제 - 16. ? 연산자

## 문제 1 — 두 숫자 더하기 (parse 에러 전파)
- 파일명: `homework_01/src/main.rs`
- 핵심 개념: `?` 연산자, `Result`

### 요구사항
- 함수 `add_strings(a: &str, b: &str) -> Result<i32, std::num::ParseIntError>` 를 정의한다.
- 두 입력을 i32 로 파싱한 뒤 합을 돌려준다.
- main 에서 다음 두 호출의 결과를 출력한다.
  - `("3", "4")` → Ok(7)
  - `("12", "x")` → Err(...)

### 예상 출력
```
("3", "4") → Ok(7)
("12", "x") → Err(...)
```

### 힌트
- `let x: i32 = a.parse()?;`, `let y: i32 = b.parse()?;`
- 두 번째 호출의 `Err(...)` 부분은 출력에 `ParseIntError` 의 Debug 형식이 들어가면 OK 입니다. 정확한 메시지까지 일치할 필요는 없습니다.

## 문제 2 — first_capital (Option ?)
- 파일명: `homework_02/src/main.rs`
- 핵심 개념: `Option<T>` 의 `?`, `chars().find`

### 요구사항
- 함수 `first_capital(s: &str) -> Option<char>` 를 작성한다.
- 입력에서 처음으로 등장하는 ASCII 대문자(A~Z)를 돌려준다. 없으면 `None`.
- main 에서 `"hello World"`, `"rust"`, `""` 세 입력의 결과를 출력한다.

### 예상 출력
```
"hello World" → Some('W')
"rust" → None
"" → None
```

### 힌트
- `s.chars().find(|c| c.is_ascii_uppercase())`.
- 또는 `let c = s.chars().find(|c| c.is_ascii_uppercase())?;` 후 `Some(c)`.

## 정답 확인

직접 풀어 본 후 [`answer/`](./answer/) 폴더의 정답과 비교해 보세요.
