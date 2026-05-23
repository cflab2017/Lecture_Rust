# 01. Rust 시작하기

Rust 는 안전성·성능·생산성을 동시에 추구하는 시스템 프로그래밍 언어입니다. C/C++ 수준의 실행 속도를 내면서도 컴파일러가 메모리 안전성을 정적으로 검증해 주기 때문에, 운영체제·웹어셈블리·게임 엔진·CLI 도구·임베디드까지 폭넓게 쓰이고 있습니다.

이 강의에서는 Rust 도구체인을 설치하고, `cargo` 로 첫 프로젝트를 만든 뒤 "Hello, World!" 를 출력하는 가장 기본적인 절차를 따라갑니다. 여기서 자리잡은 워크플로(`cargo new` → 코드 작성 → `cargo run`)는 나머지 21편 전체에서 똑같이 쓰입니다.

## 학습 목표

- 운영체제(Windows / macOS / Linux)에 맞춰 `rustup` 으로 Rust 툴체인을 설치한다.
- `cargo new` 로 새 프로젝트를 생성하고 디렉터리 구조를 이해한다.
- `cargo build` / `cargo run` / `cargo check` 의 차이를 구분해서 사용한다.
- `println!` 매크로의 기본 사용법과 `{}` 보간 문법을 익힌다.

## 핵심 개념

### 1) Rust 라는 언어

Rust 는 2015 년 1.0 이 공개된 비교적 젊은 언어지만, Stack Overflow 개발자 설문에서 여러 해 연속 "가장 사랑받는 언어" 로 뽑혀 왔습니다. 성능·안전성·동시성 세 마리 토끼를 모두 잡는다는 평을 받고 있으며, 다음과 같은 특징이 있습니다.

- **메모리 안전성**: 가비지 컬렉터 없이도 use-after-free, double-free, 데이터 레이스를 컴파일 단계에서 차단합니다.
- **제로 비용 추상화**: 트레잇·제네릭·이터레이터를 써도 런타임 오버헤드가 거의 없습니다.
- **풍부한 도구체인**: 컴파일러 `rustc`, 빌드/패키지 도구 `cargo`, 포맷터 `rustfmt`, 린터 `clippy` 가 한 묶음으로 제공됩니다.

### 2) `rustup` 으로 설치하기

`rustup` 은 Rust 버전 매니저입니다. 안정판·베타·나이틀리를 한 번에 관리할 수 있고, 업그레이드(`rustup update`)도 한 줄이면 끝납니다.

#### Windows

[https://rustup.rs](https://rustup.rs) 에서 `rustup-init.exe` 를 받아 실행합니다. 설치 중 Visual Studio Build Tools (C++ Build Tools) 가 함께 필요하다는 안내가 나오면 같이 설치해 주세요. 설치가 끝나면 새 PowerShell 창을 열어 동작을 확인합니다.

#### macOS

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

설치 후 새 터미널을 열거나 `source $HOME/.cargo/env` 를 실행해 `PATH` 를 적용합니다.

#### Linux

macOS 와 동일한 한 줄로 설치할 수 있습니다. 배포판 패키지 매니저(apt, dnf)로도 설치 가능하지만 버전이 낮은 경우가 많으니 `rustup` 사용을 권장합니다.

설치가 완료되면 다음 명령으로 동작을 확인합니다.

```sh
rustc --version   # 컴파일러 버전
cargo --version   # 패키지 매니저 버전
```

### 3) `cargo` 로 프로젝트 만들기

`cargo` 는 빌드·의존성 관리·테스트·문서 생성을 모두 담당합니다.

```sh
cargo new hello_rust       # 새 바이너리 프로젝트 (--lib 를 붙이면 라이브러리)
cd hello_rust
cargo run                  # 빌드 + 실행
cargo build                # 빌드만 (target/debug/ 에 결과물)
cargo build --release      # 최적화 빌드 (target/release/)
cargo check                # 타입 검사만 (가장 빠름)
```

`cargo new` 가 만들어 주는 기본 구조는 다음과 같습니다.

```
hello_rust/
├── Cargo.toml   # 패키지 메타데이터·의존성
├── .gitignore
└── src/
    └── main.rs  # 실행 진입점
```

### 4) `println!` 매크로

화면에 한 줄을 출력하는 가장 흔한 매크로입니다. `!` 가 붙은 이유는 함수가 아니라 **매크로** 이기 때문입니다.

- `println!("Hello, World!");` — 단순 문자열
- `println!("{} + {} = {}", 1, 2, 3);` — 위치 인수
- `println!("{name} 님 환영합니다.");` — Rust 1.58 이상에서 동일 이름 변수 직접 보간

개행 없이 출력하려면 `print!`, 표준 에러로 보내려면 `eprintln!` / `eprint!` 를 씁니다.

## 예제로 보기

### 예제 1 — `ex01_hello.rs` : Hello, World!

가장 단순한 Rust 프로그램입니다. `fn main()` 이 진입점이고, `println!` 매크로로 한 줄을 출력합니다.

```rust
// 1편 예제 1: Rust 의 "Hello, World!" 프로그램
// `cargo run --bin ex01_hello` 로 실행합니다.

fn main() {
    // println! 은 함수가 아니라 매크로이므로 이름 뒤에 `!` 가 붙습니다.
    println!("Hello, World!");
    println!("안녕하세요, Rust!");
}
```

### 예제 2 — `ex02_greet.rs` : println! 매크로로 변수 값 출력

`{}` 자리에 인수 값이 순서대로 들어갑니다. Rust 1.58 부터는 같은 이름의 변수를 `{name}` 처럼 직접 보간할 수 있습니다.

```rust
// 1편 예제 2: println! 매크로로 변수 값 출력
// `{}` 자리에 인수 값이 순서대로 들어갑니다.

fn main() {
    let name = "지수";
    let year = 2026;

    println!("이름: {}", name);
    println!("연도: {}", year);

    // 위치 인수를 여러 개 넣을 수도 있습니다.
    println!("{} 님, {} 년에 Rust 를 시작했어요!", name, year);

    // 변수를 직접 보간하는 짧은 문법 (Rust 1.58+ 안정)
    println!("{name} 님 환영합니다.");
}
```

## 자주 하는 실수

### Q. `cargo run` 을 어디서 실행해야 하나요?

A. `Cargo.toml` 이 있는 디렉터리 안에서 실행해야 합니다. `cargo` 가 상위 디렉터리를 탐색해 `Cargo.toml` 을 찾아 올라가긴 하지만, 처음에는 프로젝트 루트로 이동한 뒤 실행하는 습관이 헷갈리지 않습니다.

### Q. `println!` 뒤의 `!` 는 꼭 붙여야 하나요?

A. 네. `println` 은 **매크로** 의 이름이고, 매크로는 항상 `!` 와 함께 호출합니다. `!` 를 빼면 컴파일러가 "이런 함수가 없다" 며 에러를 냅니다.

### Q. 한국어 출력이 깨져요.

A. 소스 파일이 UTF-8 로 저장돼 있는지 먼저 확인하세요. Windows PowerShell 에서 한글이 깨진다면 콘솔 인코딩을 `chcp 65001` 로 바꾸거나 Windows Terminal 을 사용하면 해결됩니다.

### Q. `cargo build` 와 `cargo run` 의 차이는 뭔가요?

A. `cargo build` 는 빌드만 하고 결과물(`target/debug/<name>` 또는 `.exe`)을 남깁니다. `cargo run` 은 빌드 + 실행을 한 번에 합니다. 개발 중에는 `cargo run` 이 편하고, CI 나 배포 직전에는 `cargo build --release` 로 최적화 바이너리를 만듭니다.

## 정리

- Rust 는 안전성·성능·생산성을 모두 챙기는 시스템 언어로, `rustup` → `cargo` 흐름이 표준 개발 환경이다.
- 새 프로젝트는 `cargo new <name>` 으로 만들고, `Cargo.toml` 과 `src/main.rs` 가 출발점이다.
- 실행은 `cargo run`, 빠른 검사는 `cargo check`, 배포 빌드는 `cargo build --release`.
- `println!` 은 매크로이므로 `!` 가 붙으며, `{}` 로 값을 보간한다.

## 직접 해 보기

`homework/` 폴더의 과제를 풀어 보세요. 정답은 `homework/answer/` 에 있습니다.

## 다음 단원

[02. 변수·상수·섀도잉·타입 추론](../02_변수와_타입_추론/README.md) — `let` / `let mut` / `const` 의 차이와 Rust 의 타입 추론을 배웁니다.
