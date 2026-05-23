# 16. `?` 연산자와 에러 전파

`?` 연산자는 Rust 에러 처리의 일등 시민입니다. `Result` 가 `Err` 이거나 `Option` 이 `None` 이면 **함수 전체를 즉시 종료** 하면서 그 에러를 호출자로 돌려보냅니다. 깊이 중첩된 `match` 들이 한 줄로 바뀌고, `From` 트레잇과 결합하면 서로 다른 에러 타입도 매끄럽게 묶이죠. 라이브러리 코드에서 거의 모든 줄이 `?` 로 끝나는 이유입니다.

## 학습 목표

- `?` 의 동작 원리(조기 반환)를 이해한다.
- `Result<T, E>` 와 `Option<T>` 양쪽에서 `?` 를 사용한다.
- `From` 트레잇 덕에 다른 에러 타입이 자동 변환되는 것을 본다.
- `main` 의 반환 타입을 `Result<(), Box<dyn Error>>` 로 두는 패턴을 익힌다.

## 핵심 개념

### 1) `?` 가 하는 일

```rust
let n: i32 = "42".parse()?;
```

위 한 줄은 본질적으로 다음과 같습니다.

```rust
let n: i32 = match "42".parse() {
    Ok(v) => v,
    Err(e) => return Err(e.into()),
};
```

핵심:
- `Ok(v)` → 값 `v` 가 그 자리에 남는다.
- `Err(e)` → 함수 자체가 `Err(e.into())` 로 즉시 반환.

### 2) `?` 가 적용되는 자리

함수의 **반환 타입** 이 `?` 와 호환되어야 합니다.

- `Result<T, E>` 를 반환하는 함수: `?` 가 `Result<_, _>` 값에 사용 가능.
- `Option<T>` 를 반환하는 함수: `?` 가 `Option<_>` 값에 사용 가능.

`Result` 와 `Option` 을 같은 함수에서 섞고 싶다면 `ok_or` / `ok` 로 한쪽으로 통일하거나, 16편 패턴(공통 에러 타입)을 사용합니다.

### 3) `From` 자동 변환

`?` 는 단순 전파가 아닙니다. **`From<원본_에러> for 함수_에러`** 가 구현되어 있으면 자동 변환합니다.

```rust
impl From<ParseIntError> for MyError { /* ... */ }

fn parse_age(s: &str) -> Result<u8, MyError> {
    let n: i32 = s.parse()?; // ParseIntError → MyError 자동
    /* ... */
}
```

이 덕분에 `?` 는 한 줄에 여러 종류의 에러를 깔끔하게 묶을 수 있습니다.

### 4) `main` 의 반환 타입

`main` 도 `Result` 를 반환할 수 있습니다.

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let s = std::fs::read_to_string("data.txt")?;
    let n: i32 = s.trim().parse()?;
    println!("n = {n}");
    Ok(())
}
```

`Box<dyn Error>` 는 "어떤 에러든 받겠다" 라는 의미입니다. 본격적인 라이브러리는 18편의 커스텀 에러 타입을 더 선호합니다.

## 예제로 보기

### 예제 1 — `ex01_read_file.rs` : 파일 + 파싱

```rust
// 16편 예제 1: 파일 읽고 숫자 파싱 — ? 로 두 종류 에러 자연스럽게 전파
//
// `?` 는 Result/Option 이 Err/None 일 때 함수 자체를 즉시 종료하며
// 에러를 호출자에게 돌려준다. 두 다른 에러 타입이 모두 `Box<dyn Error>` 로
// 자동 변환되어 한 줄에 묶인다.

use std::error::Error;
use std::fs;

fn read_first_number(path: &str) -> Result<i32, Box<dyn Error>> {
    let content = fs::read_to_string(path)?; // io::Error
    let n: i32 = content.trim().parse()?;    // ParseIntError
    Ok(n)
}

fn main() -> Result<(), Box<dyn Error>> {
    // 임시 경로에 파일을 만들어 두고 읽는다.
    let path = std::env::temp_dir().join("lecture16_ex01.txt");
    fs::write(&path, "42")?;

    let n = read_first_number(path.to_str().expect("UTF-8 경로"))?;
    println!("읽은 숫자: {n}");

    Ok(())
}
```

### 예제 2 — `ex02_option_q.rs` : Option 의 `?`

```rust
// 16편 예제 2: Option 에도 ? 가 동작
//
// 함수 반환 타입이 Option<T> 일 때, ? 는 None 을 그대로 호출자에게 전파한다.

fn last_char_uppercase(s: &str) -> Option<char> {
    let last = s.chars().last()?;   // 빈 문자열이면 None 으로 즉시 반환
    last.to_uppercase().next()
}

fn main() {
    for s in ["hello", "rust", ""] {
        println!("{:?} → {:?}", s, last_char_uppercase(s));
    }
}
```

### 예제 3 — `ex03_from.rs` : From 자동 변환

```rust
// 16편 예제 3: ? 와 From 자동 변환
//
// `?` 는 단순히 에러를 그대로 돌려보내는 게 아니라, From 트레잇이 구현되어
// 있으면 호출자 쪽 에러 타입으로 **자동 변환** 합니다.

use std::num::ParseIntError;

#[derive(Debug)]
enum MyError {
    Parse(ParseIntError),
    OutOfRange(i32),
}

// 표준 라이브러리 에러를 우리 enum 으로 감싸기 위한 From 구현
impl From<ParseIntError> for MyError {
    fn from(e: ParseIntError) -> Self {
        MyError::Parse(e)
    }
}

fn parse_age(s: &str) -> Result<u8, MyError> {
    let n: i32 = s.parse()?; // ParseIntError → MyError 자동 변환
    if !(0..=150).contains(&n) {
        return Err(MyError::OutOfRange(n));
    }
    Ok(n as u8)
}

fn main() {
    for s in ["25", "0", "abc", "200", "-3"] {
        println!("{s:?} → {:?}", parse_age(s));
    }
}
```

## 자주 하는 실수

### Q. `?` 를 `main` 에서 쓰는데 "the `?` operator can only be used..." 에러가 납니다.

A. `fn main()` 의 반환 타입이 `()` 라서 그렇습니다. `fn main() -> Result<(), Box<dyn Error>>` 로 바꾸세요.

### Q. `Result` 함수 안에서 `Option::None` 을 `?` 로 전파하려고 합니다.

A. 그대로는 안 됩니다. `ok_or(...)` 로 먼저 `Result` 로 변환하세요.
```rust
let v = opt.ok_or("값 없음")?;
```

### Q. `From` 을 직접 구현해야 하나요?

A. 표준 에러끼리는 이미 많은 `From` 이 구현되어 있고, `Box<dyn Error>` 로 받으면 거의 자동입니다. 자기 만의 에러 타입을 만들 땐 직접 `impl From<...>` 가 필요한데, 18편의 `thiserror` 매크로로 자동화할 수 있습니다.

### Q. `unwrap` 자리에 무조건 `?` 를 쓰면 되나요?

A. 호출자가 에러를 받을 준비가 되어 있을 때만 `?` 가 의미 있습니다. 작은 스크립트의 main 에서는 `expect` 도 충분합니다 — 다만 사용자에게 보여 줄 메시지는 신중히.

## 정리

- `?` 는 Err/None 일 때 함수 자체를 조기 종료하며 에러를 전파한다.
- `Result` 와 `Option` 모두에서 동작 — 다만 함수 반환 타입이 호환되어야 한다.
- `From` 트레잇 자동 변환으로 여러 에러 타입을 자연스럽게 묶을 수 있다.
- `main` 도 `Result<(), Box<dyn Error>>` 를 반환할 수 있다.

## 직접 해 보기

`homework/` 폴더의 과제를 풀어 보세요. 정답은 `homework/answer/` 에 있습니다.

## 다음 단원

[17. panic! 과 unrecoverable 에러](../17_panic/README.md) — 회복 불가능한 에러를 다루는 방식을 정리합니다.
