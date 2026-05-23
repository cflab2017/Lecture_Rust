# 14. 트레잇 (정의·구현·기본 메서드·trait object)

트레잇(trait)은 Rust 의 다형성 도구입니다. 다른 언어의 "인터페이스" 와 비슷하지만, **기본 메서드를 가질 수 있고** 컴파일 타임 단형화 또는 런타임 동적 디스패치 모두 지원합니다. 표준 라이브러리의 `Iterator`, `Display`, `Debug`, `From` 등이 모두 트레잇이며, 라이브러리 작성의 핵심 어휘입니다.

## 학습 목표

- `trait` 키워드로 트레잇을 정의하고 타입에 `impl Trait for Type` 으로 구현한다.
- 기본 메서드(default method)를 정의해 공통 동작을 제공한다.
- `&dyn Trait` / `Box<dyn Trait>` trait object 로 런타임 다형성을 구현한다.
- 자주 쓰는 트레잇(`Debug`, `Clone`, `PartialEq`)을 `derive` 로 자동 구현한다.

## 핵심 개념

### 1) 트레잇 정의·구현

```rust
trait Summary {
    fn title(&self) -> String;            // 시그니처만
    fn summarize(&self) -> String {        // 기본 구현
        format!("({}) 요약", self.title())
    }
}

impl Summary for Article {
    fn title(&self) -> String { self.headline.clone() }
}
```

구현체는 시그니처만 있는 메서드를 반드시 채워야 합니다. 기본 메서드는 그대로 두거나 오버라이드합니다.

### 2) 트레잇 매개변수

```rust
fn print(item: &impl Summary) { /* ... */ }       // impl Trait
fn print<T: Summary>(item: &T) { /* ... */ }      // 제네릭 + bound
fn print<T>(item: &T) where T: Summary { /* ... */ } // where 절
```

세 표기는 의미가 같습니다. 가독성에 따라 선택하세요.

### 3) Trait object — 런타임 다형성

같은 트레잇을 구현한 여러 타입을 하나의 컬렉션에 담거나, 반환 타입을 동적으로 결정해야 할 때 사용합니다.

```rust
fn area(s: &dyn Shape) -> f64 { s.area() }
let shapes: Vec<Box<dyn Shape>> = vec![Box::new(Circle{...}), Box::new(Square{...})];
```

- `&dyn Trait` — 참조형 trait object
- `Box<dyn Trait>` — 힙에 담는 owning trait object

(static dispatch (`impl Trait`) 와 비교하면 호출 시 가상 함수 테이블을 거쳐 약간의 비용이 있지만, 코드가 더 유연해집니다.)

### 4) `derive` 자동 구현

| 트레잇 | 효과 |
|--------|------|
| `Debug` | `{:?}` 출력 |
| `Clone` | `.clone()` 깊은 복사 |
| `Copy`  | 비트 복사 자동 (Clone 도 필요) |
| `PartialEq` | `==` / `!=` 비교 |
| `Eq`, `Hash`, `Ord` | 정렬·해시 컨테이너 키 |
| `Default` | `T::default()` 기본값 |

### 5) 트레잇이 막힌 경우 — orphan rule

> 자기가 정의한 트레잇은 아무 타입에도 구현 가능, 자기가 정의한 타입에는 어떤 트레잇이든 구현 가능. **둘 다 외부** 인 경우는 허용 안 됨.

`String` 에 `Display` (외부+외부) 같은 조합은 직접 구현 불가입니다.

## 예제로 보기

### 예제 1 — `ex01_summary.rs` : Summary 트레잇과 기본 메서드

```rust
// 14편 예제 1: 트레잇 정의·구현·기본 메서드
//
// 트레잇은 다른 언어의 "인터페이스" 와 비슷하지만, 기본 메서드를 가질 수 있어
// 부분 구현을 강제하지 않고도 공통 동작을 제공할 수 있습니다.

trait Summary {
    // 구현이 비어 있는 메서드 — 구현체가 반드시 정의해야 함
    fn title(&self) -> String;

    // 기본 메서드 — 구현체가 오버라이드하지 않으면 이 본문이 쓰임
    fn summarize(&self) -> String {
        format!("({}) 의 요약입니다.", self.title())
    }
}

struct Article {
    headline: String,
    body: String,
}

impl Summary for Article {
    fn title(&self) -> String {
        self.headline.clone()
    }
    // summarize 는 기본 구현 사용

    // body 까지 같이 보고 싶다면 직접 override 도 가능
    //fn summarize(&self) -> String {
    //    format!("[{}] {} ...", self.headline, &self.body[..self.body.len().min(20)])
    //}
}

struct Tweet {
    user: String,
    text: String,
}

impl Summary for Tweet {
    fn title(&self) -> String {
        format!("@{}", self.user)
    }
    fn summarize(&self) -> String {
        format!("@{}: {}", self.user, self.text)
    }
}

fn main() {
    let a = Article {
        headline: String::from("Rust 1.0 출시"),
        body: String::from("Rust 가 안정판에 도달했습니다."),
    };
    let t = Tweet {
        user: String::from("rustlang"),
        text: String::from("hello from rustlang"),
    };
    println!("{}", a.summarize());
    println!("{}", t.summarize());
}
```

### 예제 2 — `ex02_shape.rs` : trait object

```rust
// 14편 예제 2: trait object (`dyn Shape`) — 런타임 다형성
//
// 트레잇을 구현한 서로 다른 타입을 같은 컬렉션에 넣고 동일 메서드를 호출하려면
// "trait object" 를 사용합니다. 보통 `Box<dyn Trait>` 또는 `&dyn Trait`.

use std::f64::consts::PI;

trait Shape {
    fn area(&self) -> f64;
    fn name(&self) -> &'static str;
}

struct Circle { r: f64 }
struct Square { side: f64 }

impl Shape for Circle {
    fn area(&self) -> f64 { PI * self.r * self.r }
    fn name(&self) -> &'static str { "Circle" }
}

impl Shape for Square {
    fn area(&self) -> f64 { self.side * self.side }
    fn name(&self) -> &'static str { "Square" }
}

fn print_area(s: &dyn Shape) {
    println!("{}: 넓이 = {:.2}", s.name(), s.area());
}

fn main() {
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { r: 3.0 }),
        Box::new(Square { side: 4.0 }),
    ];
    for s in &shapes {
        print_area(s.as_ref());
    }
}
```

### 예제 3 — `ex03_derive.rs` : derive 매크로

```rust
// 14편 예제 3: derive 매크로로 흔한 트레잇 자동 구현
//
// 자주 쓰는 트레잇은 `#[derive(...)]` 로 자동 구현 가능합니다.

#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 1, y: 2 };

    // Clone — 깊은 복사본
    let p2 = p1.clone();

    // PartialEq — == / != 비교 가능
    println!("p1 == p2 ? {}", p1 == p2);

    // Debug — {:?} / {:#?} 로 출력 가능
    println!("p1 = {:?}", p1);
    println!("p1 = {:#?}", p1); // 들여쓰기 포맷
}
```

## 자주 하는 실수

### Q. `impl Trait` 과 `dyn Trait` 의 차이는?

A. `impl Trait` 은 컴파일 타임에 단형화(monomorphization) 되어 호출 비용이 0 이지만, 함수당 한 가지 구체 타입만 사용할 수 있습니다. `dyn Trait` 은 런타임 동적 디스패치라 같은 자리에 여러 타입을 섞어 쓸 수 있지만, 호출 시 가상 함수 테이블 조회 비용이 있습니다.

### Q. `Vec<dyn Trait>` 는 왜 안 되나요?

A. trait object 는 크기를 컴파일 타임에 알 수 없는 동적 크기 타입(unsized type)이라 `Vec` 의 원소로 직접 들어가지 못합니다. 한 단계 인다이렉션을 두어 `Vec<Box<dyn Trait>>` 처럼 써야 합니다.

### Q. `derive(Clone)` 만 했는데 `clone()` 호출이 안 됩니다.

A. 구조체 내부 필드 중 `Clone` 을 구현하지 않은 타입이 있을 수 있습니다. 모든 필드가 `Clone` 이어야 자동 derive 가 통과합니다.

### Q. 트레잇 안에서 `Self` 를 반환할 수 있나요?

A. 가능합니다. 다만 trait object 로 만들려면 (object safety 규칙 때문에) `Self` 를 반환하는 메서드가 있어선 안 됩니다. 입문 단계에서는 정적 디스패치(`impl Trait`)로 처리하면 무난합니다.

## 정리

- `trait` 정의 + `impl Trait for Type` 구현이 기본 패턴.
- 기본 메서드로 공통 동작 제공, 필요 시 구현체가 오버라이드.
- `&dyn Trait` / `Box<dyn Trait>` 로 런타임 다형성을 표현.
- 흔한 트레잇은 `#[derive(...)]` 로 자동 구현 — `Debug`, `Clone`, `PartialEq` 가 자주 쓰임.

## 직접 해 보기

`homework/` 폴더의 과제를 풀어 보세요. 정답은 `homework/answer/` 에 있습니다.

## 다음 단원

[15. Result<T,E> 와 Option<T> 다루기](../../04_에러_처리/15_Result_와_Option/README.md) — 에러를 값으로 다루는 Result 와 더 많은 Option 메서드를 배웁니다.
