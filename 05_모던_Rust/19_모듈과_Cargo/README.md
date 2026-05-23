# 19. 모듈 시스템·Cargo·crates.io

코드가 늘어나면 한 파일에 모두 담을 수 없습니다. Rust 는 **모듈(`mod`)** 로 코드를 트리 형태로 나누고, **`pub`** 로 가시성을 제어하며, **Cargo** 로 외부 크레잇(crates.io)을 가져옵니다. 이번 편에서는 모듈의 인라인·파일·디렉터리 분할 패턴, `use` 와 가시성, Cargo 의존성 추가까지 정리합니다.

## 학습 목표

- `mod` 키워드로 모듈을 정의하고 `pub` 로 가시성을 제어한다.
- 모듈을 파일·디렉터리로 분할하는 두 가지 표준 패턴을 안다.
- `use` 로 경로를 단축하고 별칭을 붙인다.
- `Cargo.toml` 의 `[dependencies]` 에 외부 크레잇을 추가한다.

## 핵심 개념

### 1) 모듈은 트리

크레잇(바이너리 또는 라이브러리)은 루트 파일(`src/main.rs` 또는 `src/lib.rs`)에서 시작해 자식 모듈을 가지는 트리 구조입니다.

```
crate
├── math
│   ├── add
│   └── double
└── text
    ├── shout
    └── whisper
```

### 2) 인라인 모듈

같은 파일 안에서 `mod 이름 { ... }` 로 모듈을 만듭니다. 작은 코드에는 가장 간단한 방법.

```rust
mod math {
    pub fn add(a: i32, b: i32) -> i32 { a + b }
}
math::add(1, 2);
```

### 3) 파일·디렉터리로 분할

코드가 커지면 다음 두 방식 중 하나로 분할합니다.

**파일 분할** — `mod math;` 를 선언하면 컴파일러가 `math.rs` 를 찾습니다.

```
src/
├── main.rs           // mod math;
└── math.rs           // pub fn add(...)
```

**디렉터리 분할** — 모듈이 자식 모듈을 또 가진다면 디렉터리를 만들고 `mod.rs` 또는 `<모듈명>.rs` 에 본문을 둡니다.

```
src/
├── main.rs           // mod geometry;
└── geometry/
    ├── mod.rs        // pub mod circle; pub mod square;
    ├── circle.rs
    └── square.rs
```

(에디션 2018+ 부터는 `mod.rs` 대신 `geometry.rs` + `geometry/circle.rs` 형태도 가능합니다.)

### 4) 가시성 — `pub`

- 기본은 비공개. `pub` 를 붙여야 부모에서 보입니다.
- `pub(crate)`: 같은 크레잇 전체에서만 보임.
- `pub(super)`: 부모 모듈에서만 보임.
- `pub(in 경로)`: 지정한 경로 안에서만 보임.

### 5) `use` 로 경로 단축

```rust
use crate::math::{add, double};   // 같은 크레잇 안
use std::collections::HashMap;     // 표준 라이브러리
use rand::Rng;                     // 외부 크레잇 (의존성 추가 후)
use crate::math as m;              // 별칭
```

`super::`, `self::`, `crate::` 는 각각 부모·자기·루트를 가리키는 키워드입니다.

### 6) Cargo 의존성

`Cargo.toml`:
```toml
[dependencies]
serde = "1"
rand = { version = "0.8", features = ["std_rng"] }
```

명령으로 추가도 가능:
```sh
cargo add serde@1
cargo add rand --features std_rng
```

버전 표기 `"1"` 은 `^1.0.0` 과 동일 — semver 호환 범위를 의미합니다.

## 예제로 보기

### 예제 1 — `ex01_inline.rs` : 인라인 모듈과 가시성

```rust
// 19편 예제 1: 인라인 모듈과 pub 가시성
//
// `mod` 키워드로 같은 파일 안에서 모듈을 정의할 수 있습니다.
// 모듈 안의 항목은 기본적으로 비공개(private) — `pub` 을 붙여야 외부에서 보입니다.

mod math {
    // pub 가 없으면 모듈 밖에서 호출 불가
    fn _internal() -> i32 { 42 }

    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn double(x: i32) -> i32 {
        x * 2
    }
}

fn main() {
    // 외부에서는 `모듈명::함수명` 으로 호출
    let sum = math::add(3, 4);
    let twice = math::double(sum);
    println!("(3+4)*2 = {twice}");

    // math::_internal();  // ❌ 비공개라 호출 불가
}
```

### 예제 2 — `ex02_pub_use.rs` : use 로 경로 단축

```rust
// 19편 예제 2: use 로 경로 줄이기
//
// 자주 쓰는 항목은 `use` 로 가져와 짧은 이름만 쓰면 가독성이 좋아집니다.

mod text {
    pub fn shout(s: &str) -> String {
        s.to_uppercase() + "!"
    }

    pub fn whisper(s: &str) -> String {
        format!("({})", s.to_lowercase())
    }
}

// 모듈 함수들을 현재 스코프로 가져오기
use text::{shout, whisper};

// 자기 모듈을 별칭으로 가져올 수도 있다.
use text as tt;

fn main() {
    println!("{}", shout("hello"));
    println!("{}", whisper("Quiet"));

    // 별칭으로도 호출
    println!("{}", tt::shout("hi"));
}
```

### 예제 3 — `ex03_nested.rs` : 중첩 모듈

```rust
// 19편 예제 3: 중첩 모듈과 super
//
// 모듈은 트리 구조라 자식 → 부모를 `super::` 로 접근할 수 있습니다.
// 또한 같은 깊이의 형제 모듈끼리도 부모를 거쳐 접근합니다.

mod geometry {
    pub mod circle {
        use std::f64::consts::PI;

        pub fn area(r: f64) -> f64 {
            // 부모의 형제(square)에 접근하려면 super::square::...
            PI * r * r
        }
    }

    pub mod square {
        pub fn area(side: f64) -> f64 {
            side * side
        }
    }
}

// 깊은 경로는 use 로 줄이기
use geometry::{circle, square};

fn main() {
    println!("circle area (r=2)  = {:.2}", circle::area(2.0));
    println!("square area (s=3)  = {:.2}", square::area(3.0));
}
```

## 자주 하는 실수

### Q. `mod math;` 라고 적었는데 컴파일러가 파일을 못 찾습니다.

A. `src/main.rs` 가 루트라면 컴파일러는 `src/math.rs` 또는 `src/math/mod.rs` 를 찾습니다. 자식 모듈에서 또 다른 자식이 있다면 디렉터리 구조가 일치해야 합니다.

### Q. `pub use` 는 뭔가요?

A. 다른 곳의 항목을 **자기 모듈의 공개 API 로 재노출** 합니다. 라이브러리에서 깊은 내부 모듈의 항목을 루트에서 바로 쓰게 하고 싶을 때 사용합니다.

### Q. 외부 크레잇 버전 명시는 어떻게 읽나요?

A. semver(major.minor.patch) 규약을 따릅니다. `"1"` 은 `^1.0.0` 으로 1.x 의 최신 호환 버전, `"=1.2.3"` 은 정확히 1.2.3, `"~1.2"` 는 1.2.x 입니다.

### Q. `cargo build` vs `cargo build --release` 의 차이?

A. 기본은 디버그 빌드(target/debug/) — 빠른 컴파일, 느린 실행. `--release` 는 최적화 빌드(target/release/) — 느린 컴파일, 빠른 실행. 배포 직전에 사용.

## 정리

- 모듈은 트리 구조 — `mod` 로 정의, `pub` 로 가시성 제어.
- 작으면 인라인, 커지면 파일·디렉터리로 분할.
- `use` 로 경로를 줄이고 `as` 로 별칭.
- 외부 크레잇은 `Cargo.toml` 의 `[dependencies]` 또는 `cargo add` 로 추가.

## 직접 해 보기

`homework/` 폴더의 과제를 풀어 보세요. 정답은 `homework/answer/` 에 있습니다.

## 다음 단원

[20. 단위 테스트·통합 테스트·doctest](../20_테스트/README.md) — Rust 의 내장 테스트 도구로 코드를 안전하게 다듬는 방법을 배웁니다.
