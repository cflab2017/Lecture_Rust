# 12. 열거형 + Option<T> 입문

열거형(enum)은 "여러 가지 모양 중 정확히 하나" 를 표현하는 타입입니다. 구조체가 AND(필드 1 **와** 필드 2)라면, 열거형은 OR(variant A **또는** variant B). 각 variant 는 데이터를 가질 수 있어서 단순 라벨부터 복잡한 합 타입(sum type)까지 표현할 수 있습니다. 표준 라이브러리의 `Option<T>` 가 대표적인 enum 입니다 — Rust 에 null 이 없는 비결.

## 학습 목표

- `enum` 으로 여러 variant 를 정의하고 인스턴스를 만든다.
- variant 가 데이터를 담는 다양한 형태(tuple/struct/unit)를 안다.
- `impl` 로 enum 에 메서드를 붙인다.
- `Option<T>` 의 의미와 `Some`/`None` 을 익히고 null 안전을 이해한다.

## 핵심 개념

### 1) enum 의 모양

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),  // 튜플형
    V6(String),          // 단일 필드
    Loopback,            // 데이터 없음
}
```

같은 enum 안에서도 variant 마다 데이터 모양이 다를 수 있습니다.

### 2) impl 로 메서드 붙이기

```rust
impl Coin {
    fn value_in_cents(&self) -> u32 {
        match self { /* ... */ }
    }
}
```

`match` 는 다음 편에서 자세히 다루지만, 위 패턴이 enum + 메서드의 가장 흔한 모양입니다.

### 3) `Option<T>` — null 안전

```rust
pub enum Option<T> {
    None,
    Some(T),
}
```

값이 있을 수도, 없을 수도 있는 자리에 정확히 이 타입을 씁니다. "어떤 값이 없을 수 있다" 는 사실이 타입에 그대로 드러나기 때문에, 사용자는 `match` 나 `if let` 으로 명시적으로 처리해야 합니다.

```rust
fn first_even(nums: &[i32]) -> Option<i32> {
    for &n in nums {
        if n % 2 == 0 { return Some(n); }
    }
    None
}
```

### 4) `Option` 기본 처리 메서드

| 메서드 | 동작 |
|--------|------|
| `unwrap()` | `Some` 이면 값, `None` 이면 패닉 (안전한 자리에서만) |
| `unwrap_or(default)` | `None` 일 때 기본값 |
| `unwrap_or_else(\|\| ...)` | None 일 때 클로저로 기본값 계산 |
| `map(\|x\| ...)` | `Some` 안의 값을 변환 |
| `and_then(\|x\| ...)` | `Some` 일 때 다른 Option 반환 (체이닝) |
| `is_some()`, `is_none()` | 존재 여부만 |
| `?` 연산자 | None 을 호출자로 전파 (16편) |

### 5) C# / Java 의 `null` 과 비교

- C# / Java: 거의 모든 참조 타입이 null 이 될 수 있어, 사용 시 매번 검사 필요.
- Rust: 기본적으로 null 없음. 부재 가능성은 **반드시** `Option<T>` 로 명시.

이 차이가 NullReferenceException / NullPointerException 류 버그를 완전히 봉쇄합니다.

## 예제로 보기

### 예제 1 — `ex01_basic.rs` : IpAddr enum

```rust
// 12편 예제 1: 열거형 — "이 중 하나" 를 표현하는 타입
//
// 각 variant 는 데이터를 가질 수 있고, 데이터 모양도 자유롭다.

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),    // 튜플형 데이터
    V6(String),            // 한 필드
    Loopback,              // 데이터 없는 variant
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let server = IpAddr::V6(String::from("::1"));
    let lb = IpAddr::Loopback;

    println!("home   = {:?}", home);
    println!("server = {:?}", server);
    println!("lb     = {:?}", lb);
}
```

### 예제 2 — `ex02_coin.rs` : Coin enum 과 메서드

```rust
// 12편 예제 2: 데이터를 가지지 않는 단순 enum + 메서드
//
// match 의 전형적인 사용 예. 다음 편(13)에서 매칭을 자세히 다룹니다.

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

impl Coin {
    fn value_in_cents(&self) -> u32 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
}

fn main() {
    let purse = [Coin::Penny, Coin::Nickel, Coin::Dime, Coin::Quarter, Coin::Dime];

    let total: u32 = purse.iter().map(|c| c.value_in_cents()).sum();
    println!("지갑 = {:?}", purse);
    println!("총합 = {total} 센트");
}
```

### 예제 3 — `ex03_option.rs` : Option<T>

```rust
// 12편 예제 3: Option<T> — null 안전 타입
//
// 표준 라이브러리의 Option 은 사실 enum 입니다:
//   pub enum Option<T> { None, Some(T) }
// Rust 에 null 이 없는 비결입니다.

fn find_even(nums: &[i32]) -> Option<i32> {
    for &n in nums {
        if n % 2 == 0 {
            return Some(n);
        }
    }
    None
}

fn main() {
    let result1 = find_even(&[1, 3, 5, 8, 9]);
    let result2 = find_even(&[1, 3, 5]);

    // 가장 기본적인 처리: match
    match result1 {
        Some(n) => println!("처음 짝수: {n}"),
        None => println!("짝수 없음"),
    }
    // unwrap_or: None 일 때 기본값
    let v = result2.unwrap_or(-1);
    println!("두 번째 결과(기본 -1) = {v}");

    // ? 연산자는 16편에서 자세히 다룹니다.
}
```

## 자주 하는 실수

### Q. enum 도 derive(Debug) 가 필요한가요?

A. `{:?}` 로 출력하려면 필요합니다. 보통은 enum 정의 위에 `#[derive(Debug, Clone, PartialEq)]` 정도를 자주 붙입니다.

### Q. `Option::None` 이 다른 언어의 null 과 다른가요?

A. 다릅니다. null 은 "어떤 참조 타입이든 비어 있을 수 있다" 라 사용 시 매번 검사가 필요합니다. `None` 은 `Option<T>` 타입에 한정되므로, 타입을 통해 "이 자리는 비어 있을 수 있다" 가 컴파일러에 드러납니다.

### Q. `let v = opt.unwrap();` 을 막 써도 되나요?

A. 위험합니다. `None` 일 때 패닉이 납니다. 명백히 `Some` 인 자리 (예: 방금 생성한 값) 외에는 `match`/`if let`/`?`/`unwrap_or` 등을 사용하세요.

### Q. variant 끼리 같은 데이터 모양을 공유하고 싶어요.

A. 가능합니다. 같은 enum 안에서 variant 마다 다른 모양을 자유롭게 섞을 수 있습니다 (예제 1 의 `IpAddr` 처럼 V4 는 튜플, V6 는 단일 필드, Loopback 은 데이터 없음).

## 정리

- `enum` 은 "여러 모양 중 하나" 의 타입 — 각 variant 가 데이터를 가질 수 있다.
- `impl` 로 enum 에 메서드를 붙일 수 있다.
- `Option<T>` 는 표준 라이브러리 enum으로 null 안전을 제공한다.
- `unwrap` 은 안전이 명백한 곳만, 그 외엔 `match`/`if let`/`?` 를 사용.

## 직접 해 보기

`homework/` 폴더의 과제를 풀어 보세요. 정답은 `homework/answer/` 에 있습니다.

## 다음 단원

[13. 패턴 매칭 (match·if let·while let)](../13_패턴_매칭/README.md) — enum 과 Option 을 자유롭게 분해하는 강력한 도구를 배웁니다.
