# 10. String 과 &str 깊게 — UTF-8·인덱싱

Rust 의 문자열은 처음 만나면 "왜 이렇게 까다롭지?" 싶습니다. 다른 언어처럼 `s[0]` 으로 첫 글자를 꺼낼 수 없고, `String` 과 `&str` 도 헷갈리죠. 그 이유는 명확합니다 — Rust 의 문자열은 **항상 UTF-8** 이고, 한 "글자" 가 1~4 바이트일 수 있기 때문입니다. 이번 편에서 그 원칙과 일상적인 처리 패턴을 정리합니다.

## 학습 목표

- `String` 과 `&str` 의 메모리 모델을 이해한다.
- 문자열 결합 4 가지(`push_str`/`push`/`+`/`format!`)를 상황에 맞게 고른다.
- UTF-8 바이트 길이와 유니코드 문자 개수의 차이를 안다.
- 문자열 인덱싱이 막혀 있는 이유와 안전한 순회 방법(`chars()`/`bytes()`)을 안다.

## 핵심 개념

### 1) `String` vs `&str`

| 타입       | 소유? | 가변? | 메모리                            |
|-----------|-------|-------|-----------------------------------|
| `String`  | O     | O(가능) | 힙에 데이터, 스택에 (ptr, len, cap) |
| `&str`    | X     | X     | 다른 곳의 UTF-8 바이트를 가리키는 (ptr, len) |
| `&'static str` | X | X | 바이너리 안의 정적 데이터 |

함수 매개변수로는 `&str` 이 거의 정답입니다 — `String`, `&String`, 문자열 리터럴 모두 호환되니까요.

### 2) 결합 방법 4 가지

| 방법 | 코드 | 비고 |
|------|------|------|
| `push_str` | `s.push_str("...");` | 슬라이스를 덧붙임 |
| `push` | `s.push('!');` | 한 글자 |
| `+` | `let c = a + &b;` | 좌변 String 이동, 우변 &str |
| `format!` | `let c = format!("{a}{b}");` | 새 String, 인수 소유권 유지 |

### 3) UTF-8 의 함정

```rust
let s = "한"; // UTF-8 로 3 바이트
s.len();          // 3 (바이트)
s.chars().count(); // 1 (유니코드 스칼라)
```

`len()` 은 항상 바이트 수입니다. 글자 수가 필요하면 `chars().count()`. 그래픽 그래핌(예: 결합 이모지) 단위가 필요하면 외부 크레잇 `unicode-segmentation` 을 씁니다.

### 4) 왜 인덱싱이 안 될까?

`s[0]` 이 가능하면 사용자는 1바이트만 가져오려 할 텐데, 그게 유효한 UTF-8 문자가 아닐 수 있습니다. Rust 는 **항상 유효한 UTF-8** 만 다루도록 강제하므로 직접 인덱싱을 금지하고 명시적인 메서드(`chars()`, `bytes()`, 슬라이싱)만 허용합니다.

### 5) 슬라이싱

`&s[a..b]` 는 바이트 단위입니다. 멀티바이트 문자 중간을 잘라내면 런타임 패닉. 안전하게는 `s.char_indices()` 로 문자 경계를 알아낸 뒤 자릅니다.

## 예제로 보기

### 예제 1 — `ex01_combine.rs` : 결합 방법

```rust
// 10편 예제 1: 문자열 결합 방법
//
// push_str / push / + 연산자 / format! 매크로

fn main() {
    // 가변 String 에 직접 덧붙이기
    let mut greeting = String::from("Hello");
    greeting.push_str(", ");      // 문자열 슬라이스를 추가
    greeting.push('R');           // 한 글자(char)
    greeting.push_str("ust!");
    println!("push 계열: {greeting}");

    // + 연산자 — 좌변은 String 소유권, 우변은 &str
    let a = String::from("안녕, ");
    let b = String::from("세계!");
    let ab = a + &b; // a 는 이동, b 는 빌림
    println!("+ 연산자: {ab}");

    // format! — 새 String 을 만들고 어떤 인수도 소유권을 가져가지 않음
    let name = String::from("Rust");
    let lang = String::from("Korean");
    let sentence = format!("{} 강의를 {} 로 듣고 있어요.", name, lang);
    println!("format!: {sentence}");
    println!("(원본 그대로 사용 가능) name={name}, lang={lang}");
}
```

### 예제 2 — `ex02_iterate.rs` : 순회와 인덱싱

```rust
// 10편 예제 2: 문자열 순회와 인덱싱
//
// 문자열은 UTF-8 가변 길이 인코딩이라 직접 인덱싱이 막혀 있습니다.
// 대신 `chars()` (유니코드 스칼라) 와 `bytes()` (원시 바이트) 를 사용합니다.

fn main() {
    let s = String::from("Rust 한");

    // s[0] 같은 인덱싱은 컴파일 에러
    // let c = s[0]; // ❌

    // 문자(char) 단위 순회
    print!("chars: ");
    for c in s.chars() {
        print!("{c} ");
    }
    println!();

    // 바이트 단위 순회 (UTF-8 raw bytes)
    print!("bytes: ");
    for b in s.bytes() {
        print!("{b} ");
    }
    println!();

    println!("byte len  = {}", s.len());
    println!("char count = {}", s.chars().count());

    // 안전한 슬라이싱: 문자 경계를 알 때만 사용
    // "Rust" 의 4 바이트만 가져오기
    let head = &s[..4];
    println!("head = {head}");
}
```

### 예제 3 — `ex03_owned_vs_borrowed.rs` : 소유와 빌림 변환

```rust
// 10편 예제 3: String(소유) ↔ &str(빌림) 변환

fn describe(s: &str) -> String {
    format!("입력 \"{s}\" 의 길이는 {} 바이트", s.len())
}

fn main() {
    let owned: String = String::from("자유");
    let lit: &str = "리터럴";

    // &String → &str 은 deref 로 자동
    println!("{}", describe(&owned));
    println!("{}", describe(lit));

    // &str → String (소유 사본 만들기)
    let copied: String = lit.to_string();
    let copied2: String = String::from(lit);
    println!("copied = {copied}, copied2 = {copied2}");

    // String → &str (슬라이스 만들기)
    let borrowed: &str = owned.as_str();
    println!("borrowed = {borrowed}");
}
```

## 자주 하는 실수

### Q. `s.len() == 3` 인데 화면에 한 글자만 보입니다.

A. `len()` 은 UTF-8 바이트 수입니다. 한국어 한 글자는 보통 3 바이트라 그렇게 나옵니다. 글자 수가 필요하면 `s.chars().count()` 를 쓰세요.

### Q. `&s[0..2]` 가 패닉이 납니다.

A. 문자열 슬라이싱은 바이트 단위인데, UTF-8 문자 경계가 아닌 곳을 잘랐기 때문입니다. 안전하게 잘라내려면 `s.char_indices().nth(N).map(|(i, _)| &s[..i])` 같은 패턴을 씁니다.

### Q. `String::new()` 와 `String::from("")` 차이가 있나요?

A. 결과는 모두 빈 String 으로 동일합니다. 단지 의도 표현 차이입니다 — 곧 채울 빈 버퍼면 `new()`, 리터럴 기반이면 `from`.

### Q. `+` 와 `format!` 중 어느 쪽이 좋나요?

A. 결합할 조각이 2~3 개고 좌변 String 의 소유권을 넘겨도 괜찮다면 `+` 가 빠릅니다. 그 외에는 `format!` 이 안전하고 가독성도 좋습니다.

## 정리

- `String` 은 소유·가변, `&str` 은 빌림. 함수 매개변수는 보통 `&str`.
- 결합은 `push_str`/`push`/`+`/`format!` 네 가지를 상황에 맞게.
- `len()` 은 바이트, `chars().count()` 가 문자 수.
- 직접 인덱싱은 금지, 슬라이싱은 바이트 단위라 문자 경계 주의.

## 직접 해 보기

`homework/` 폴더의 과제를 풀어 보세요. 정답은 `homework/answer/` 에 있습니다.

## 다음 단원

[11. 구조체 (named·tuple·unit)](../../03_타입_시스템/11_구조체/README.md) — 도메인 타입을 정의하는 방법을 배웁니다.
