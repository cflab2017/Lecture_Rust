# 17. `panic!` 과 unrecoverable 에러

Rust 의 에러는 두 갈래입니다. 호출자가 의미 있게 다룰 수 있는 **회복 가능(recoverable)** 에러는 `Result<T, E>` 로 표현하고, "이건 잘못됐다 — 더 진행하면 안 된다" 라는 **회복 불가(unrecoverable)** 상황은 `panic!` 으로 처리합니다. 이번 편에서는 `panic!`, 인덱스 OOB, `unwrap`/`expect` 의 동작과 디버깅 팁을 정리합니다.

## 학습 목표

- `panic!` 매크로의 의미와 종료 방식을 이해한다.
- 인덱스 OOB·정수 오버플로·`unwrap` 등 흔한 패닉 원인을 안다.
- `expect("이유")` 메시지로 디버깅 가능성을 높이는 습관을 들인다.
- `RUST_BACKTRACE=1` 로 스택 트레이스를 확인한다.

## 핵심 개념

### 1) `panic!` 의 동작

`panic!("메시지")` 가 호출되면 기본 동작은 다음과 같습니다.
1. 콘솔에 에러 메시지와 위치를 출력.
2. 현재 스레드의 스택을 **언와인드(unwind)** 하며 모든 지역 변수를 `drop`.
3. 스레드가 메인이면 프로세스가 종료(exit code ≠ 0).

`Cargo.toml` 의 `[profile]` 에 `panic = "abort"` 를 두면 언와인드 없이 즉시 종료해서 바이너리 크기가 줄어듭니다.

### 2) 흔한 패닉 원인

| 원인 | 예 | 안전한 대안 |
|------|----|-------------|
| 인덱스 OOB | `v[10]` 인데 길이 3 | `v.get(10)` (Option) |
| 정수 오버플로 (디버그) | `u8 + u8` | `checked_add`, `wrapping_add` |
| `unwrap`/`expect` | `None.unwrap()` | `match`/`?`/`unwrap_or` |
| 0 나눗셈 (정수) | `n / 0` | 사전 검사 또는 `checked_div` |

### 3) `unwrap` vs `expect`

```rust
let n: i32 = "42".parse().unwrap();           // 실패 시 일반 메시지
let n: i32 = "42".parse().expect("입력은 항상 정수"); // 실패 시 우리 메시지
```

`expect` 의 메시지는 "왜 이 자리에서 실패할 수 없는가" 를 적어 두면 다음 사람이 이해하기 쉽습니다.

### 4) 백트레이스 보기

```sh
RUST_BACKTRACE=1 cargo run --bin ex01_explicit
RUST_BACKTRACE=full cargo run --bin ex01_explicit
```

호출 스택 전체를 볼 수 있어 디버깅에 큰 도움이 됩니다.

### 5) 회복 가능 vs 회복 불가 — 어떻게 고르나?

| 상황 | 처리 |
|------|------|
| 사용자 입력·외부 데이터·I/O | `Result` + `?` (회복 가능) |
| 라이브러리 내부의 무결성 검증 (불변식 위반) | `panic!` |
| 명백히 도달 불가한 분기 | `unreachable!()` |
| 아직 구현하지 않은 분기 | `todo!()` |

라이브러리 코드는 거의 항상 `Result` 를 돌려 주는 게 정중한 매너입니다. 결정은 호출자에게 맡기세요.

## 예제로 보기

> 본 예제들은 학습 목적으로 `catch_unwind` 로 패닉을 잡아 깔끔한 출력을 만듭니다. 실제 코드에서 `catch_unwind` 는 거의 쓰지 않습니다 — 보통은 그냥 죽게 둡니다.

### 예제 1 — `ex01_explicit.rs` : panic! 직접 호출

```rust
// 17편 예제 1: panic! 직접 호출
//
// "회복 불가능한" 상황을 만나면 panic!("메시지") 로 프로그램을 즉시 중단합니다.
// 이 예제는 정상 흐름과 패닉 흐름을 모두 보여 주기 위해 catch_unwind 로 감쌉니다.
// (실제 코드에서 catch_unwind 는 거의 쓰지 않습니다 — 보통은 그냥 죽게 둡니다.)

fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("0으로 나눌 수 없습니다 (a={a})");
    }
    a / b
}

fn main() {
    // 기본 패닉 훅을 끄면 스택 트레이스가 안 보이고 콘솔이 깨끗합니다.
    std::panic::set_hook(Box::new(|_| {}));

    println!("정상: 10/2 = {}", divide(10, 2));

    // 패닉을 잡아 메시지를 추출 (학습 용도)
    let result = std::panic::catch_unwind(|| divide(10, 0));
    if let Err(payload) = result {
        let msg = payload
            .downcast_ref::<String>()
            .cloned()
            .or_else(|| payload.downcast_ref::<&str>().map(|s| s.to_string()))
            .unwrap_or_else(|| String::from("(메시지 추출 실패)"));
        println!("패닉 발생! 메시지: {msg}");
    }
}
```

### 예제 2 — `ex02_index_oob.rs` : 인덱스 OOB

```rust
// 17편 예제 2: 인덱스 범위 초과 패닉
//
// 배열·벡터의 범위 밖 인덱스 접근은 컴파일러가 잡지 못하고 런타임에 패닉합니다.
// 안전한 대안은 `.get(i)` — 결과를 Option 으로 돌려줍니다.

fn main() {
    std::panic::set_hook(Box::new(|_| {}));

    let v = vec![10, 20, 30];
    println!("v[0] = {}, v[2] = {}", v[0], v[2]);

    // v[10] 은 인덱스 OOB → 패닉
    let r = std::panic::catch_unwind(|| v.clone()[10]);
    if r.is_err() {
        println!("v[10] 접근은 패닉 — 범위를 벗어났기 때문입니다.");
    }

    // 안전한 대안: Option 으로 받기
    println!("v.get(2)  = {:?}", v.get(2));
    println!("v.get(10) = {:?}", v.get(10));
}
```

### 예제 3 — `ex03_expect.rs` : expect 메시지

```rust
// 17편 예제 3: expect 와 RUST_BACKTRACE
//
// `unwrap` 은 "값이 없으면 패닉" 의 일반 메시지만 띄웁니다.
// `expect("이유")` 는 패닉 시 메시지를 함께 출력해 디버깅에 큰 도움이 됩니다.
// 또한 환경변수 RUST_BACKTRACE=1 로 실행하면 호출 스택을 볼 수 있습니다.

fn main() {
    std::panic::set_hook(Box::new(|_| {}));

    // 안전한 자리에서의 expect — 메시지가 "왜 안전한지" 의 문서가 된다.
    let s = "42";
    let n: i32 = s.parse().expect("이 자리는 항상 정수 — 위에서 검증함");
    println!("n = {n}");

    // 실패 시 expect 메시지 확인
    let r = std::panic::catch_unwind(|| {
        let _: i32 = "abc".parse().expect("입력은 항상 정수여야 함");
    });
    if r.is_err() {
        println!("expect 패닉 발생 — 메시지가 디버그에 큰 도움.");
    }

    println!("환경변수 RUST_BACKTRACE=1 로 실행하면 스택 트레이스를 확인할 수 있습니다.");
}
```

## 자주 하는 실수

### Q. 패닉이 발생하면 catch 로 잡을 수 있나요?

A. `std::panic::catch_unwind` 로 잡을 수 있지만, 일반 예외 처리 메커니즘이 아닙니다. FFI 경계나 백그라운드 작업이 메인 스레드를 죽이지 못하게 하는 정도의 특수 용도입니다. 일반 코드는 `Result` 와 `?` 를 씁니다.

### Q. `panic!` 이 발생하면 다른 스레드도 죽나요?

A. 패닉이 발생한 **스레드만** 죽습니다. 메인 스레드가 죽으면 전체 프로세스 종료. 21편의 스레드 강의에서 다시 다룹니다.

### Q. 정수 오버플로는 항상 패닉인가요?

A. **디버그 빌드** 에서는 패닉, **릴리스 빌드** 에서는 wrapping 으로 동작합니다. 의도가 wrapping 이면 `wrapping_add`, 검출이 필요하면 `checked_add` (Option) / `overflowing_add` (튜플) 를 명시해 쓰세요.

### Q. 라이브러리에서 panic 을 써도 되나요?

A. 호출자가 회복 불가능한 상황(불변식 위반 등)에서는 가능하지만, **사용자 입력·외부 데이터** 에 대해 패닉하는 것은 매우 무례합니다. 보통은 `Result` 로 정보를 돌려주고 사용자가 결정하게 합니다.

## 정리

- `panic!` 은 회복 불가 상황의 마지막 수단 — 스레드 종료 + drop.
- 인덱스 OOB / 정수 오버플로(디버그) / `unwrap` 이 흔한 원인.
- `unwrap` 대신 `expect("이유")` 로 메시지를 남기면 디버깅이 쉬워진다.
- 라이브러리는 거의 항상 `Result` 를 반환 — 결정은 호출자에게.

## 직접 해 보기

`homework/` 폴더의 과제를 풀어 보세요. 정답은 `homework/answer/` 에 있습니다.

## 다음 단원

[18. 커스텀 에러 타입 (thiserror·From)](../18_커스텀_에러/README.md) — 도메인에 맞는 에러 타입을 정의해 ? 와 자연스럽게 결합하는 방법을 배웁니다.
