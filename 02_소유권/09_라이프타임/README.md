# 09. 라이프타임 입문 (`'a` 명시·생략 규칙)

라이프타임(lifetime) 은 "참조가 유효한 기간" 을 컴파일러에게 알려 주는 라벨입니다. 6~8편에서 본 모든 참조에는 사실 라이프타임이 붙어 있었지만, 대부분 컴파일러가 자동으로 채워 주었을 뿐입니다. 이번 편에서는 라이프타임을 **언제·왜 명시해야 하는지**, 그리고 컴파일러가 어떤 규칙으로 생략을 허용하는지 정리합니다.

## 학습 목표

- 라이프타임이 왜 필요한지 댕글링 참조 차단의 관점에서 이해한다.
- 함수 시그니처에 `'a` 를 명시하는 기본 패턴을 익힌다.
- 참조를 가지는 구조체에 라이프타임 파라미터를 붙인다.
- 라이프타임 생략 규칙 3가지를 기억하고 언제 생략이 안 되는지 안다.

## 핵심 개념

### 1) 왜 라이프타임이 필요한가

```rust
fn longest(a: &str, b: &str) -> &str { /* ... */ } // ❌ 컴파일 안 됨
```

반환된 참조가 `a` 의 것인지 `b` 의 것인지 컴파일러가 알 수 없습니다. 호출 후 한 입력의 수명이 끝나면 반환된 참조가 댕글링 될 수 있죠. 그래서 라이프타임을 명시합니다.

```rust
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str { ... }
```

`'a` 는 "이 함수가 다루는 참조들이 공통으로 유효한 기간" 을 가리키는 라벨입니다.

### 2) 함수 시그니처의 라이프타임

- `<'a>` 로 라이프타임 파라미터를 선언.
- 입력 참조와 출력 참조에 동일 라벨을 붙이면 "출력 라이프타임은 입력 라이프타임의 교집합 이상" 으로 묶입니다.

### 3) 구조체의 라이프타임

참조를 필드로 두는 구조체는 라이프타임 파라미터가 **필수** 입니다.

```rust
struct Excerpt<'a> { part: &'a str }
```

`Excerpt` 인스턴스는 `part` 가 가리키는 값보다 오래 살 수 없습니다.

### 4) 라이프타임 생략 규칙 3가지

| # | 규칙 |
|---|------|
| 1 | 각 입력 참조에는 자기만의 라이프타임이 부여된다. |
| 2 | 입력 참조가 **정확히 하나** 면 그 라이프타임이 출력에 자동 적용된다. |
| 3 | 첫 입력이 `&self` / `&mut self` 이면 그 라이프타임이 출력에 자동 적용된다. |

세 규칙으로도 출력 라이프타임이 결정되지 않으면 컴파일러는 명시를 요구합니다.

### 5) 특별한 라이프타임 `'static`

`'static` 은 프로그램이 종료될 때까지 사는 참조를 의미합니다. 문자열 리터럴(`"hello"`)의 타입이 `&'static str` 입니다.

## 예제로 보기

### 예제 1 — `ex01_longest.rs` : 명시적 라이프타임

```rust
// 9편 예제 1: 명시적 라이프타임 'a 를 가진 함수
//
// 두 슬라이스 중 하나를 돌려주는 함수의 반환 라이프타임은
// "둘 다 살아 있는 동안" 이어야 합니다.

fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

fn main() {
    let s1 = String::from("길어요");
    let s2 = String::from("짧음");
    let result = longest(&s1, &s2);
    println!("긴 것: {result}");

    // 두 입력의 스코프가 다를 때
    let s_outer = String::from("outer scope");
    {
        let s_inner = String::from("inner scope-long");
        let r = longest(&s_outer, &s_inner);
        println!("긴 것(스코프 안): {r}");
        // s_inner 가 살아 있는 범위 내에서 r 도 유효
    }
}
```

### 예제 2 — `ex02_struct.rs` : 참조를 가지는 구조체

```rust
// 9편 예제 2: 참조를 가지는 구조체와 라이프타임
//
// 필드에 참조를 두는 구조체는 라이프타임 파라미터가 필요합니다.
// 구조체 인스턴스는 그 필드가 가리키는 값보다 오래 살 수 없습니다.

struct Excerpt<'a> {
    part: &'a str,
}

impl<'a> Excerpt<'a> {
    fn announce(&self, prefix: &str) -> &str {
        println!("[{prefix}] 발췌 미리보기: {}", self.part);
        self.part
    }
}

fn main() {
    let article = String::from("Rust 의 라이프타임은 참조가 유효한 기간을 의미합니다.");
    let first_sentence = article.split('.').next().expect("문장 없음");

    let ex = Excerpt { part: first_sentence };
    let kept = ex.announce("INFO");

    println!("kept = {kept}");
    // article 이 살아 있는 동안 ex.part / kept 모두 유효
}
```

### 예제 3 — `ex03_elision.rs` : 생략 규칙

```rust
// 9편 예제 3: 라이프타임 생략 규칙 (Lifetime Elision)
//
// 자주 등장하는 패턴은 컴파일러가 라이프타임을 자동으로 채워 줍니다.
// 규칙:
// 1) 각 참조 입력은 자기만의 라이프타임을 부여받는다.
// 2) 입력이 정확히 하나면 그 라이프타임을 출력에 적용한다.
// 3) 메서드의 첫 인수가 &self / &mut self 이면 그 라이프타임을 출력에 적용한다.

// (1) 생략 가능 — 입력 참조 1개 → 출력에 그대로 적용
fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}

// (2) 생략 불가 — 입력이 두 개라 어느 쪽 라이프타임을 출력에 적용할지 모름
//     명시가 필요하다.
fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

// (3) 메서드 — &self 라이프타임이 자동 적용
struct Holder<'a> { value: &'a str }

impl<'a> Holder<'a> {
    fn value(&self) -> &str { self.value } // 라이프타임 생략됨
}

fn main() {
    let s = String::from("Rust language is fast");
    println!("first_word = {}", first_word(&s));
    println!("longer     = {}", longer("aaa", "bb"));

    let h = Holder { value: &s };
    println!("holder     = {}", h.value());
}
```

## 자주 하는 실수

### Q. 라이프타임이 값의 수명을 늘려 주는 건가요?

A. 아니요. 라이프타임은 **이미 결정된** 수명을 컴파일러에 알려 주는 **라벨** 일 뿐입니다. 명시한다고 실제 수명이 늘어나거나 줄어들지 않습니다.

### Q. `'a` 가 꼭 'a 여야 하나요?

A. 관례입니다. `'static`, `'long`, `'ctx` 처럼 의미 있는 이름을 붙이는 것도 좋습니다. 다만 단순한 경우에는 알파벳 하나면 충분합니다.

### Q. 함수마다 `<'a>` 를 적는 게 번거롭습니다.

A. 8편의 `first_word` 처럼 입력 참조가 하나뿐이면 생략 규칙으로 자동 처리됩니다. 정말 필요한 자리에서만 명시하면 됩니다.

### Q. `'static` 을 남발해도 되나요?

A. 안 됩니다. `'static` 으로 강제하면 호출자가 `'static` 데이터(주로 문자열 리터럴)만 넘길 수 있어 함수의 범용성이 크게 떨어집니다. 보통은 `'a` 같은 제네릭 라이프타임을 씁니다.

## 정리

- 라이프타임은 참조의 유효 기간을 표시하는 라벨 — 댕글링을 차단한다.
- 함수 시그니처에서 `<'a>` 와 `&'a T` 로 명시한다.
- 참조 필드를 가지는 구조체는 라이프타임 파라미터가 필수.
- 입력 참조 1개·`&self` 등의 경우는 생략 규칙으로 자동 채워진다.

## 직접 해 보기

`homework/` 폴더의 과제를 풀어 보세요. 정답은 `homework/answer/` 에 있습니다.

## 다음 단원

[10. String 과 &str 깊게 — UTF-8·인덱싱](../10_String_과_str/README.md) — 문자열 처리의 진짜 내부를 들여다봅니다.
