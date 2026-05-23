# 03. 기본 타입 (정수·실수·bool·char·tuple·array)

Rust 의 타입 시스템은 엄격합니다. 정수도 i8 부터 i128 까지 비트 수를 명시할 수 있고, 문자열 한 글자는 `char` 하나에 유니코드 코드 포인트 전체가 들어갑니다. 이번 편에서는 스칼라 타입(정수·실수·bool·char) 과 복합 타입(튜플·배열)을 살펴봅니다.

## 학습 목표

- 정수 타입의 비트 수·부호 유무를 구분해서 사용한다.
- `f32`/`f64`, `bool`, `char` 의 기본 동작을 이해한다.
- 튜플과 배열의 정의·접근·구조 분해 문법을 익힌다.
- 한국어 문자열의 "바이트 길이" 와 "문자 개수" 가 다르다는 점을 안다.

## 핵심 개념

### 1) 정수 타입

| 부호  | 비트 8 | 16 | 32(기본) | 64 | 128 | 포인터 폭 |
|-------|--------|-----|---------|-----|------|-----------|
| 있음  | `i8`   | `i16` | `i32` | `i64` | `i128` | `isize` |
| 없음  | `u8`   | `u16` | `u32` | `u64` | `u128` | `usize` |

- 정수 리터럴은 기본 `i32`, 실수는 `f64`.
- `usize` / `isize` 는 포인터 폭(32/64bit)에 맞게 결정되며 길이·인덱스에 쓰임.
- **디버그 빌드** 에서 오버플로는 패닉, **릴리스 빌드** 에서는 wrapping. 의도적 wrapping 이 필요하면 `wrapping_add` 같은 메서드를 사용한다.

### 2) 실수 타입

- `f32`(단정밀도), `f64`(배정밀도, 기본).
- 정수/실수는 자동 변환이 없으므로 명시 캐스팅 `as`: `let f = i as f64;`
- 부동소수의 비교는 동등성보다 `(a - b).abs() < EPS` 패턴이 안전.

### 3) `bool` 과 `char`

- `bool` 은 `true` / `false` 두 값.
- `char` 는 작은따옴표, **유니코드 스칼라 1개** 를 담는 4 바이트 값. 한글·이모지 모두 한 `char`.
- 문자열(`&str`/`String`)은 큰따옴표. `s.len()` 은 **바이트 수**, `s.chars().count()` 가 **문자 수**.

### 4) 튜플과 배열

- 튜플 `(T1, T2, ...)`: 서로 다른 타입 묶음, 고정 길이. 인덱스는 `.0`, `.1`.
- 배열 `[T; N]`: 같은 타입의 고정 길이 묶음. 인덱스는 `arr[0]`. 길이는 `arr.len()`.
- 동적 길이가 필요하면 다음 편 이후의 `Vec<T>` 로.

## 예제로 보기

### 예제 1 — `ex01_numbers.rs` : 정수·실수·bool

```rust
// 3편 예제 1: 정수·실수·bool

fn main() {
    // 정수 타입은 i / u + 비트 수: i8, i16, i32(기본), i64, i128, isize
    // 부호 없는 정수는 u8 ~ u128, usize.
    let signed: i32 = -42;
    let unsigned: u64 = 100_000_000;
    let pointer_size: usize = 1024; // 배열/벡터 길이 표현에 사용

    println!("i32  = {signed}");
    println!("u64  = {unsigned}");
    println!("usize= {pointer_size}");

    // 실수 타입은 f32, f64(기본)
    let pi: f64 = 3.141_592_653_589_793;
    let half = 0.5_f32;
    println!("pi   = {pi}");
    println!("half = {half}");

    // 산술 연산자: + - * / %
    // 정수 나눗셈은 버림 (소수점 버려짐)
    let div = 7 / 2;       // 3
    let rem = 7 % 2;       // 1
    let fdiv = 7.0 / 2.0;  // 3.5
    println!("7/2 = {div}, 7%2 = {rem}, 7.0/2.0 = {fdiv}");

    // bool — true / false
    let on: bool = true;
    let off = !on;
    println!("on = {on}, off = {off}");
}
```

### 예제 2 — `ex02_chars.rs` : char 와 유니코드

```rust
// 3편 예제 2: char 는 유니코드 스칼라 한 글자 (4 바이트)

fn main() {
    let a: char = 'A';
    let han: char = '한';
    let emoji: char = '🦀';
    let escape: char = '\n';

    println!("ASCII : {a}");
    println!("한글  : {han}");
    println!("이모지: {emoji}");
    println!("개행 이스케이프 문자도 char: {}", escape as u32);

    // char 는 작은따옴표, 문자열(`&str`) 은 큰따옴표
    let s: &str = "안녕";
    println!("문자열: {s}, 바이트 길이: {}", s.len());
    // ⚠️ s.len() 은 "바이트" 길이입니다. 한국어 한 글자는 보통 3 바이트입니다.
    println!("문자 개수: {}", s.chars().count());
}
```

### 예제 3 — `ex03_tuple_array.rs` : 튜플과 배열

```rust
// 3편 예제 3: 튜플과 배열

fn main() {
    // 튜플: 서로 다른 타입을 묶을 수 있는 고정 길이 컬렉션
    let point: (i32, i32, &str) = (3, 5, "origin");

    // 인덱스 접근은 `.0`, `.1` ...
    println!("x = {}, y = {}, name = {}", point.0, point.1, point.2);

    // 구조 분해 (destructuring)
    let (x, y, name) = point;
    println!("분해: {x}, {y}, {name}");

    // 배열: 같은 타입의 고정 길이 컬렉션 [T; N]
    let scores: [u32; 5] = [80, 90, 75, 100, 60];
    println!("첫 점수: {}", scores[0]);
    println!("점수 개수: {}", scores.len());

    // 동일 값으로 채우기: [값; 개수]
    let zeros = [0u8; 4];
    println!("zeros = {:?}", zeros);
}
```

## 자주 하는 실수

### Q. `i32` 와 `u32` 를 자동으로 섞어 쓸 수 있나요?

A. 아니요. Rust 는 정수 간 자동 변환을 하지 않습니다. 필요한 곳에 `as` 캐스팅(`x as i64`)을 명시해야 합니다. 부주의한 좁힘 변환은 정보를 잃을 수 있어 컴파일러가 막고 있습니다.

### Q. 정수 오버플로는 어떻게 처리되나요?

A. 디버그 빌드에서는 **패닉**, 릴리스 빌드에서는 **wrapping** 으로 동작이 다릅니다. 의도가 wrapping 이라면 `wrapping_add` 등 명시 메서드를 쓰고, 검출이 필요하면 `checked_add` (Option) 또는 `overflowing_add` (튜플) 를 사용합니다.

### Q. `"안녕".len()` 이 6 이 나오는 이유는?

A. `len()` 은 UTF-8 **바이트 수** 를 돌려줍니다. 한국어 한 글자는 보통 3 바이트라 두 글자면 6 입니다. 글자 수가 필요하면 `chars().count()` 를 사용하세요. (그래픽 그래핌은 더 복잡하지만 입문 단계에선 `chars()` 로 충분합니다.)

### Q. 배열의 길이를 변수로 정할 수 있나요?

A. 컴파일 타임에 알아야 합니다. 런타임에 크기가 변하는 컬렉션은 `Vec<T>` 를 사용하세요. (모듈/Vec 은 이후 강의에서 자세히 다룹니다.)

## 정리

- 정수는 `iN`/`uN`, 실수는 `f32`/`f64`, 기본 추론은 `i32` 와 `f64`.
- 타입 간 변환은 자동이 아닌 명시 캐스팅(`as`).
- `char` 는 유니코드 한 글자, `&str.len()` 은 바이트 수임에 주의.
- 튜플은 이종 묶음 `(T1, T2, ...)`, 배열은 동종 고정 길이 `[T; N]`.

## 직접 해 보기

`homework/` 폴더의 과제를 풀어 보세요. 정답은 `homework/answer/` 에 있습니다.

## 다음 단원

[04. 제어 흐름 (if·loop·while·for·break value)](../04_제어_흐름/README.md) — Rust 의 조건 분기와 반복 표현식을 배웁니다.
