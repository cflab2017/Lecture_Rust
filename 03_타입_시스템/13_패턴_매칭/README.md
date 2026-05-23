# 13. 패턴 매칭 (match·if let·while let)

`match` 는 Rust 의 분기 도구 중 가장 강력합니다. C/Java 의 `switch` 와 비슷해 보이지만 **전수성(exhaustiveness)** 검사가 있어서, 모든 가능성을 다루지 않으면 컴파일이 막힙니다. 또한 패턴 안에서 데이터를 분해·바인딩할 수 있어 enum·Option 처리에 자연스럽게 어울리죠. `if let`/`while let` 은 한 variant 만 짧게 다루는 약식 문법입니다.

## 학습 목표

- `match` 의 분기·바인딩·와일드카드·가드를 모두 사용한다.
- 전수성 검사 덕에 분기 누락이 컴파일 단계에서 잡힌다는 사실을 안다.
- `if let` 으로 단일 variant 의 코드를 짧게 쓴다.
- `while let` 으로 Option 을 돌려주는 반복을 자연스럽게 처리한다.

## 핵심 개념

### 1) `match` 의 기본 구조

```rust
match value {
    패턴1 => 식1,
    패턴2 => 식2,
    _    => 기본식, // 나머지 전부
}
```

- 모든 분기가 같은 타입의 값을 돌려줘야 한다(표현식).
- 전수성: 모든 가능성을 다루지 않으면 컴파일 에러. `_` 와일드카드가 안전망.

### 2) 데이터 바인딩

```rust
enum Coin { Quarter(String), /* ... */ }

match c {
    Coin::Quarter(state) => println!("from {state}"),
    _ => (),
}
```

variant 의 내부 데이터에 이름을 붙여 분기 안에서 사용합니다.

### 3) 가드(guard) `if`

```rust
match score {
    s if s >= 90 => 'A',
    s if s >= 80 => 'B',
    _ => 'F',
}
```

패턴과 별개의 추가 조건을 붙일 수 있습니다. 가드가 통과한 첫 분기만 선택됩니다.

### 4) `if let`

```rust
if let Some(n) = maybe {
    println!("{n}");
}
```

`match maybe { Some(n) => ..., None => () }` 와 같지만 짧고 명확합니다. `else` 절도 가능.

### 5) `while let`

```rust
while let Some(top) = stack.pop() {
    println!("{top}");
}
```

매칭이 실패하는 순간 루프가 끝납니다. `Option` 을 돌려주는 메서드와 자연스럽게 결합합니다.

## 예제로 보기

### 예제 1 — `ex01_match.rs` : match, 바인딩, 가드

```rust
// 13편 예제 1: match — 패턴 분기, 바인딩, 와일드카드, 가드

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(String), // 어느 주의 쿼터인지
}

fn describe(c: &Coin) -> String {
    match c {
        Coin::Penny => String::from("1 센트"),
        Coin::Nickel => String::from("5 센트"),
        Coin::Dime => String::from("10 센트"),
        // variant 의 내부 데이터를 바인딩
        Coin::Quarter(state) => format!("25 센트 ({state} 주)"),
    }
}

fn grade(score: i32) -> char {
    // _ 와일드카드 + 가드(if 조건)
    match score {
        s if s >= 90 => 'A',
        s if s >= 80 => 'B',
        s if s >= 70 => 'C',
        _ => 'F',
    }
}

fn main() {
    println!("{}", describe(&Coin::Penny));
    println!("{}", describe(&Coin::Quarter(String::from("Alaska"))));

    for s in [95, 82, 73, 50] {
        println!("{s} → {}", grade(s));
    }
}
```

### 예제 2 — `ex02_if_let.rs` : if let

```rust
// 13편 예제 2: if let — 한 가지 variant 만 다룰 때 간결한 문법
//
// match 와 비교해 "나머지는 무시" 가 자연스러울 때 적합합니다.

fn main() {
    let maybe = Some(42);

    // match 로 처리
    match maybe {
        Some(n) => println!("(match) 값 = {n}"),
        None => {}
    }

    // 같은 동작을 if let 으로 짧게
    if let Some(n) = maybe {
        println!("(if let) 값 = {n}");
    }

    // else 도 가능
    let nothing: Option<i32> = None;
    if let Some(n) = nothing {
        println!("값 = {n}");
    } else {
        println!("(if let else) 값 없음");
    }
}
```

### 예제 3 — `ex03_while_let.rs` : while let

```rust
// 13편 예제 3: while let — 패턴이 매칭되는 동안 반복
//
// Vec 의 pop() 처럼 Option 을 돌려주는 메서드와 자연스럽게 결합됩니다.

fn main() {
    let mut stack = vec![1, 2, 3, 4];

    // 스택이 빌 때까지 마지막 원소를 꺼낸다.
    while let Some(top) = stack.pop() {
        println!("꺼냄: {top}");
    }
    println!("최종 = {:?}", stack);
}
```

## 자주 하는 실수

### Q. `match` 에 `_` 분기를 무조건 두면 안 되나요?

A. `_` 가 있으면 전수성은 만족되지만, **새 variant 가 추가됐을 때 컴파일러가 알려 주지 않습니다**. 잠재적 미스 처리 위험이 있으니 가능하면 구체적인 variant 를 나열하세요.

### Q. `if let` 과 `match` 중 어느 쪽이 좋나요?

A. variant 가 2 개 이상이고 모두 처리해야 한다면 `match`. 한 variant 만 다루고 나머지는 무시하면 `if let`. 가독성이 결정합니다.

### Q. `match` 안에서 새 변수를 만들 수 있나요?

A. 분기 본문은 일반 코드 블록이라 `let` 도 자유롭게 쓸 수 있습니다. 패턴 자체에서 바인딩한 변수는 분기 본문에서만 유효합니다.

### Q. 가드(`if`) 도 전수성 검사를 합니까?

A. 가드는 패턴 매칭 후에 추가 조건을 검사하는 것이라 전수성에는 영향을 주지 않습니다. 가드가 모두 false 가 되어도 컴파일러는 알 수 없으므로, 가드 사용 시에는 `_` 같은 안전망을 마지막에 두는 게 좋습니다.

## 정리

- `match` 는 전수성·바인딩·가드를 모두 지원하는 강력한 분기 도구.
- `if let` 은 한 variant 만 짧게 다루는 약식 문법.
- `while let` 은 Option 을 돌려주는 메서드와 결합되는 반복 패턴.
- 와일드카드 `_` 는 편하지만, 새 variant 추가 시 알림을 잃을 수 있다는 점을 주의.

## 직접 해 보기

`homework/` 폴더의 과제를 풀어 보세요. 정답은 `homework/answer/` 에 있습니다.

## 다음 단원

[14. 트레잇 (정의·구현·기본 메서드·trait object)](../14_트레잇/README.md) — Rust 의 다형성 도구 트레잇을 본격적으로 다룹니다.
