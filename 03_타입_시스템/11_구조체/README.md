# 11. 구조체 (named·tuple·unit 구조체)

구조체는 여러 데이터를 하나의 타입으로 묶어 도메인을 표현하는 가장 흔한 도구입니다. Rust 의 구조체는 세 가지 형태가 있습니다 — 이름 있는 필드를 가진 **named struct**, 위치만 있는 **tuple struct**, 필드가 없는 **unit struct**. 각자 어울리는 자리가 다르고, `impl` 블록으로 메서드를 붙여 객체 지향처럼 사용할 수 있습니다.

## 학습 목표

- named/tuple/unit 세 종류 구조체를 정의하고 인스턴스를 만든다.
- `impl` 블록에 메서드와 연관 함수를 정의해 사용한다.
- `Self` 와 `&self` / `&mut self` / `self` 의 차이를 안다.
- `#[derive(Debug)]` 등 자주 쓰는 derive 매크로의 의미를 안다.

## 핵심 개념

### 1) named-field 구조체

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

let r = Rectangle { width: 30, height: 20 };
println!("{}", r.width);
```

필드 이름이 있으니 의도가 분명합니다. 가장 자주 쓰입니다.

### 2) 튜플 구조체

```rust
struct Color(u8, u8, u8);
let red = Color(255, 0, 0);
red.0 == 255;
```

필드 이름 없이 위치로 접근합니다. 같은 모양의 타입이라도 이름이 다르면 호환되지 않아 **새 타입(newtype) 패턴** 에 유용합니다.

### 3) 유닛 구조체

```rust
struct AlwaysReady;
```

필드 없는 타입. 트레잇 구현용 마커, 상태 머신의 "상태", 빈 핸들 등에 쓰입니다.

### 4) `impl` 블록과 메서드

```rust
impl Rectangle {
    fn new(w: u32, h: u32) -> Self { Self { width: w, height: h } }
    fn area(&self) -> u32 { self.width * self.height }
}
```

- `Self` 는 구현 대상 타입(`Rectangle`)을 가리키는 별칭.
- 첫 인수가 `self` 계열이면 **메서드**, 없으면 **연관 함수**(생성자 등).
- `&self` 는 빌림, `&mut self` 는 가변 빌림, `self` 는 소유권을 가져가는 메서드.

### 5) `derive` 매크로

자주 쓰는 트레잇은 매크로로 자동 구현 가능합니다. 입문 단계에서 가장 흔한 것이 `Debug`.

```rust
#[derive(Debug)]
struct Point { x: i32, y: i32 }
let p = Point { x: 1, y: 2 };
println!("{:?}", p); // Point { x: 1, y: 2 }
```

## 예제로 보기

### 예제 1 — `ex01_named.rs` : Rectangle 과 메서드

```rust
// 11편 예제 1: named-field 구조체와 메서드(impl)
//
// 구조체는 도메인 개념을 타입으로 표현하는 가장 흔한 도구입니다.

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 연관 함수(Associated function) — Self::new 같은 생성자
    fn new(w: u32, h: u32) -> Self {
        Self { width: w, height: h }
    }

    // 메서드 — 첫 인수가 &self
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 다른 사각형과 비교하는 메서드
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let r1 = Rectangle::new(30, 20);
    let r2 = Rectangle { width: 10, height: 5 }; // 필드 직접 초기화도 가능

    println!("{:?} 의 넓이 = {}", r1, r1.area());
    println!("{:?} 는 {:?} 를 포함? {}", r1, r2, r1.can_hold(&r2));
}
```

### 예제 2 — `ex02_tuple.rs` : 튜플 구조체

```rust
// 11편 예제 2: 튜플 구조체 — 이름은 있지만 필드 이름은 없음

#[derive(Debug)]
struct Color(u8, u8, u8);

#[derive(Debug)]
struct Point(f64, f64);

impl Color {
    fn brightness(&self) -> u16 {
        // u8 끼리 더하면 오버플로 위험 → u16 으로 확장
        self.0 as u16 + self.1 as u16 + self.2 as u16
    }
}

fn main() {
    let red = Color(255, 0, 0);
    let origin = Point(0.0, 0.0);

    println!("red = {:?}, brightness = {}", red, red.brightness());
    println!("origin = ({}, {})", origin.0, origin.1);

    // 튜플 구조체끼리는 타입이 다르면 서로 호환되지 않음
    // let x: Color = origin; // ❌
}
```

### 예제 3 — `ex03_unit.rs` : 유닛 구조체와 상태 타입

```rust
// 11편 예제 3: 유닛 구조체 — 필드가 없는 타입
//
// 트레잇 구현용 마커, 상태 머신 표식 등에 자주 쓰입니다.

struct AlwaysReady; // 필드 없음

impl AlwaysReady {
    fn ping(&self) -> &'static str {
        "pong"
    }
}

// "상태" 자체를 타입으로 표현하는 패턴
struct Connected;
struct Disconnected;

fn describe<T>(_state: &T) -> &'static str
where
    T: HasName,
{
    T::name()
}

trait HasName {
    fn name() -> &'static str;
}

impl HasName for Connected {
    fn name() -> &'static str { "Connected" }
}

impl HasName for Disconnected {
    fn name() -> &'static str { "Disconnected" }
}

fn main() {
    let svc = AlwaysReady;
    println!("svc.ping() = {}", svc.ping());

    let on = Connected;
    let off = Disconnected;
    println!("on  → {}", describe(&on));
    println!("off → {}", describe(&off));
}
```

## 자주 하는 실수

### Q. `impl` 안에서 `Self` 와 `self` 가 헷갈립니다.

A. 대문자 `Self` 는 **타입** 의 이름, 소문자 `self` 는 **인스턴스** 의 이름입니다. `Self { x: 0 }` 는 "이 타입의 인스턴스를 새로 만들어", `self.x` 는 "현재 인스턴스의 x".

### Q. 같은 구조체에 `impl` 을 여러 번 써도 되나요?

A. 가능합니다. 코드를 모듈처럼 나눠 작성할 때 자주 씁니다. 컴파일러는 모두 모아 하나의 정의로 취급합니다.

### Q. `#[derive(Debug)]` 가 없으면 `{:?}` 가 컴파일되지 않습니다.

A. 맞습니다. `Debug` 트레잇 구현이 필요합니다. 보통은 `derive` 로 자동 구현하고, 커스텀이 필요할 때만 손으로 `impl std::fmt::Debug` 합니다.

### Q. 필드 일부만 새로 채우고 나머지는 기존 값을 쓰고 싶어요.

A. **구조체 갱신 문법** `..` 을 씁니다.
```rust
let r2 = Rectangle { width: 100, ..r1 };
```
다만 비-Copy 필드가 포함되면 `r1` 이 이동될 수 있다는 점을 유의하세요.

## 정리

- 세 종류: named-field / tuple / unit 구조체. 상황에 맞게 고른다.
- `impl` 블록으로 메서드와 연관 함수를 붙인다.
- `Self`(타입) ≠ `self`(인스턴스). `&self` / `&mut self` / `self` 를 의도에 맞게.
- `#[derive(Debug, Clone, ...)]` 로 흔한 트레잇을 자동 구현한다.

## 직접 해 보기

`homework/` 폴더의 과제를 풀어 보세요. 정답은 `homework/answer/` 에 있습니다.

## 다음 단원

[12. 열거형 + Option<T> 입문](../12_열거형과_Option/README.md) — "여러 가지 모양 중 하나" 를 표현하는 enum 과 null 안전 타입 Option 을 배웁니다.
