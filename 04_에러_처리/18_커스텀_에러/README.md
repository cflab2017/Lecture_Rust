# 18. 커스텀 에러 타입 (thiserror·From)

`Box<dyn Error>` 는 빠르고 편하지만 라이브러리가 커지면 한계가 옵니다. 호출자가 에러의 종류를 분간해서 다르게 처리하려면 **도메인 전용 에러 타입** 이 필요하죠. 이번 편에서는 `thiserror` 크레잇으로 깔끔한 enum 에러를 정의하고, `#[from]` 으로 `From` 트레잇을 자동 구현해 `?` 와 자연스럽게 결합하는 방법을 배웁니다.

## 학습 목표

- 표준 라이브러리 `std::error::Error` 트레잇의 의미를 안다.
- `thiserror::Error` 매크로로 enum 에러 타입을 정의한다.
- `#[error("...")]` 속성으로 `Display` 메시지를 자동 생성한다.
- `#[from]` 으로 `From` 을 자동 구현해 `?` 변환을 매끄럽게 만든다.

## 핵심 개념

### 1) `std::error::Error` 트레잇

표준 라이브러리의 거의 모든 에러는 이 트레잇을 구현합니다. 직접 구현할 수도 있지만 부담스러워서 매크로의 도움을 받습니다.

### 2) `thiserror` 가 뭘 해 주나

- `#[derive(Error)]` 로 `Error` 트레잇 자동 구현.
- `#[error("메시지 {0}")]` 로 `Display` 자동 구현.
- `#[from]` 로 `From<원본_에러>` 자동 구현 → `?` 자동 변환.

`Cargo.toml`:
```toml
[dependencies]
thiserror = "1"
```

### 3) 기본 정의 패턴

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MyError {
    #[error("정수 파싱 실패: {0}")]
    Parse(#[from] std::num::ParseIntError),
    #[error("값이 범위 밖: {0}")]
    OutOfRange(i32),
}
```

- 외부 에러를 감싸는 variant 에는 `#[from]`.
- 도메인 고유의 variant 에는 데이터를 직접 들고 메시지에 `{0}` 등으로 보간.

### 4) 라이브러리 vs 애플리케이션

| 경우 | 권장 |
|------|------|
| 라이브러리 (재사용) | 도메인 별 커스텀 에러 enum (`thiserror`) |
| 애플리케이션 (실행 바이너리) | `anyhow::Result` 같이 단일 동적 에러 — 본 강의는 이 부분은 다루지 않음 |

라이브러리 사용자는 어떤 종류의 실패가 가능한지를 **타입으로** 보고 분기할 수 있어야 합니다.

### 5) `?` 와 결합

```rust
fn read_and_double(path: &str) -> Result<i32, AppError> {
    let s = std::fs::read_to_string(path)?; // io::Error → AppError
    let n: i32 = s.trim().parse()?;         // ParseIntError → AppError
    Ok(n * 2)
}
```

세 줄로 두 출처의 에러가 한 타입으로 통합됩니다.

## 예제로 보기

### 예제 1 — `ex01_define.rs` : 커스텀 에러 정의

```rust
// 18편 예제 1: thiserror 로 커스텀 에러 enum 정의
//
// `#[derive(Error)]` 와 `#[error("...")]` 만으로 Display + Error 트레잇이 자동 구현됩니다.

use thiserror::Error;

#[derive(Debug, Error)]
enum AgeError {
    #[error("나이는 0 이상이어야 합니다 (현재 {0})")]
    Negative(i32),
    #[error("나이가 너무 큽니다 (최대 150, 현재 {0})")]
    TooLarge(i32),
}

fn validate_age(n: i32) -> Result<u8, AgeError> {
    if n < 0 {
        return Err(AgeError::Negative(n));
    }
    if n > 150 {
        return Err(AgeError::TooLarge(n));
    }
    Ok(n as u8)
}

fn main() {
    for n in [25, -3, 200] {
        match validate_age(n) {
            Ok(v) => println!("{n} → 유효: {v}"),
            // {} 는 Display, {:?} 는 Debug — thiserror 가 둘 다 깔끔하게 만들어 줌
            Err(e) => println!("{n} → 에러: {e}"),
        }
    }
}
```

### 예제 2 — `ex02_from.rs` : `#[from]` 으로 ? 자동 변환

```rust
// 18편 예제 2: #[from] 으로 From 자동 구현 + ? 자동 변환
//
// 외부 에러 타입(예: ParseIntError) 을 우리 enum 의 한 variant 로 감쌀 때
// `#[from]` 한 줄이면 From 트레잇이 자동으로 만들어집니다.

use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug, Error)]
enum MyError {
    #[error("정수 파싱 실패: {0}")]
    Parse(#[from] ParseIntError),     // ?  ParseIntError → MyError 자동
    #[error("값이 범위 밖: {0}")]
    OutOfRange(i32),
}

fn parse_positive(s: &str) -> Result<u32, MyError> {
    let n: i32 = s.parse()?;          // ParseIntError 가 자동으로 MyError 로
    if n < 0 {
        return Err(MyError::OutOfRange(n));
    }
    Ok(n as u32)
}

fn main() {
    for s in ["42", "abc", "-3"] {
        match parse_positive(s) {
            Ok(v) => println!("{s} → Ok({v})"),
            Err(e) => println!("{s} → Err: {e}"),
        }
    }
}
```

### 예제 3 — `ex03_app.rs` : 두 출처 에러 통합

```rust
// 18편 예제 3: 두 종류 외부 에러를 한 커스텀 에러로 통합
//
// 실전에서는 IO 와 파싱처럼 다른 출처의 에러가 한 함수에서 만납니다.
// 같은 커스텀 에러로 묶어 두면 호출자가 처리하기 편합니다.

use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug, Error)]
enum AppError {
    #[error("IO 실패: {0}")]
    Io(#[from] std::io::Error),
    #[error("파싱 실패: {0}")]
    Parse(#[from] ParseIntError),
}

fn read_and_double(path: &str) -> Result<i32, AppError> {
    let s = std::fs::read_to_string(path)?;   // io::Error → AppError
    let n: i32 = s.trim().parse()?;           // ParseIntError → AppError
    Ok(n * 2)
}

fn main() {
    let path = std::env::temp_dir().join("lecture18_ex03.txt");
    std::fs::write(&path, "10").expect("임시 파일 작성");
    let path_str = path.to_str().expect("UTF-8 경로");

    match read_and_double(path_str) {
        Ok(v) => println!("결과 = {v}"),
        Err(e) => println!("실패: {e}"),
    }

    // 일부러 잘못된 내용을 써서 파싱 실패도 보여 준다.
    std::fs::write(&path, "not-a-number").expect("임시 파일 재작성");
    match read_and_double(path_str) {
        Ok(v) => println!("결과 = {v}"),
        Err(e) => println!("실패: {e}"),
    }
}
```

## 자주 하는 실수

### Q. `#[from]` 을 두 variant 에 같은 타입으로 붙이면?

A. From 구현이 충돌해서 컴파일 에러가 납니다. 같은 외부 에러 타입을 두 variant 에 매핑하고 싶다면 한 variant 에만 `#[from]` 을 붙이고 나머지는 직접 `From` 을 구현하지 마세요.

### Q. `Display` 메시지에 필드 이름을 쓸 수 있나요?

A. 네. struct-like variant 라면 `{field}` 로 직접 보간할 수 있습니다.
```rust
#[error("나이 {age} 가 범위 {min}~{max} 밖")]
OutOfRange { age: i32, min: i32, max: i32 },
```

### Q. `Box<dyn Error>` 와 커스텀 에러 enum 중 어느 쪽이 좋나요?

A. 빠른 프로토타이핑·바이너리 main 에서는 `Box<dyn Error>` 가 편합니다. 라이브러리·재사용 코드에서는 호출자가 종류를 분간할 수 있어야 하므로 커스텀 enum 이 권장됩니다.

### Q. 에러 메시지의 원인 체인은 어떻게?

A. `#[source]` 속성으로 원인 에러를 가리킬 수 있습니다. `?` 변환 시 자동으로 source 를 설정해 주는 `#[from]` 이 보통 더 편합니다.

## 정리

- 도메인별 커스텀 에러 enum 을 만들면 호출자가 종류별로 처리 가능.
- `thiserror::Error` + `#[error("...")]` + `#[from]` 으로 보일러플레이트가 거의 사라진다.
- `?` 가 자동 변환을 해 주므로 여러 출처의 에러를 한 줄에 깔끔하게 합칠 수 있다.
- 라이브러리는 커스텀 enum, 짧은 애플리케이션 main 은 `Box<dyn Error>` 도 무난.

## 직접 해 보기

`homework/` 폴더의 과제를 풀어 보세요. 정답은 `homework/answer/` 에 있습니다.

## 다음 단원

[19. 모듈 시스템·Cargo·crates.io](../../05_모던_Rust/19_모듈과_Cargo/README.md) — 코드를 모듈로 나누고 외부 크레잇을 받아 쓰는 방법을 배웁니다.
