# 22. async/await + tokio 입문

`async`/`await` 는 한 스레드 안에서 여러 비동기 작업(I/O, 타이머, 네트워크 호출)을 효율적으로 다루기 위한 Rust 의 모델입니다. 표준 라이브러리만으로는 런타임이 없어서 실제로 비동기 코드를 돌리려면 **tokio** 같은 런타임이 필요하죠. 이번 편에서는 `async fn` 의 의미, `.await` 가 하는 일, `tokio::join!` 으로 동시성을 끌어내는 패턴, 그리고 비동기 채널까지 한 번에 정리합니다.

## 학습 목표

- `async fn` 이 즉시 실행되지 않고 `Future` 를 돌려준다는 점을 이해한다.
- `.await` 와 런타임의 역할을 안다.
- `#[tokio::main]` 매크로로 비동기 main 을 작성한다.
- `tokio::join!` 으로 여러 작업을 동시에 진행시킨다.
- `tokio::sync::mpsc` 로 비동기 메시지 채널을 사용한다.

## 핵심 개념

### 1) `async fn` 과 `Future`

```rust
async fn greet(name: &str) {
    println!("안녕, {name}");
}
```

이 함수를 호출해도 본문이 즉시 실행되지 않습니다. 대신 **`Future` 라는 "할 일" 을 묶은 값** 이 돌아옵니다.

```rust
let fut = greet("Rust"); // 아직 아무 일도 안 일어남
fut.await;               // 이때 런타임이 본문을 실행
```

### 2) `.await` 가 하는 일

- `Future` 가 끝날 때까지 그 자리에 멈춥니다.
- "멈춘다" 는 게 스레드를 점유한 채로 기다리는 게 **아닙니다** — 런타임이 같은 스레드에서 다른 Future 로 잠깐 전환했다가 돌아옵니다.

### 3) 런타임 — tokio

Rust 표준 라이브러리에는 비동기 런타임이 없습니다. 가장 인기 있는 런타임이 [`tokio`](https://tokio.rs) 입니다.

`Cargo.toml`:
```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

`#[tokio::main]` 매크로가 비동기 main 을 가능하게 해 줍니다.

```rust
#[tokio::main]
async fn main() {
    /* ... */
}
```

### 4) `tokio::join!` — 동시성

여러 Future 를 동시에 진행시키고 모두 끝나기를 기다립니다.

```rust
let (a, b) = tokio::join!(task1(), task2());
```

순차 await 와 비교해 합치는 시점만 다릅니다 — 진행은 동시에.

### 5) `tokio::task::spawn` — 백그라운드 task

`thread::spawn` 의 비동기 버전. 새 task 를 띄우고 JoinHandle 을 돌려줍니다. (스레드보다 훨씬 가볍습니다 — 한 프로세스에 수만 개도 무방합니다.)

### 6) `tokio::sync::mpsc`

비동기 채널입니다. `send().await`, `recv().await` 가 가능하다는 점만 21편의 `std::sync::mpsc` 와 다릅니다.

## 예제로 보기

### 예제 1 — `ex01_simple.rs` : 첫 async fn

```rust
// 22편 예제 1: 첫 async fn 과 tokio::main 매크로
//
// `async fn` 은 호출 즉시 실행되지 않고 `Future` 를 돌려줍니다.
// `.await` 가 그 Future 를 실제로 진행시키는 트리거입니다.
// `#[tokio::main]` 매크로는 tokio 런타임을 자동으로 띄워 줍니다.

use tokio::time::{sleep, Duration};

async fn greet(name: &str) {
    println!("안녕, {name}!");
    sleep(Duration::from_millis(50)).await;
    println!("{name} 에게 작별 인사.");
}

#[tokio::main]
async fn main() {
    println!("프로그램 시작");
    greet("Rust").await;
    greet("Tokio").await;
    println!("프로그램 종료");
}
```

### 예제 2 — `ex02_join.rs` : 동시 실행

```rust
// 22편 예제 2: tokio::join! 으로 여러 비동기 작업을 동시에 실행
//
// 두 작업이 순차로 실행되면 100ms + 100ms = 200ms 가 걸리지만,
// `join!` 으로 동시에 실행하면 약 100ms 만에 끝납니다.

use std::time::Instant;
use tokio::time::{sleep, Duration};

async fn task(id: u32, ms: u64) -> u32 {
    println!("[{id}] 시작");
    sleep(Duration::from_millis(ms)).await;
    println!("[{id}] 종료");
    id
}

#[tokio::main]
async fn main() {
    let start = Instant::now();

    // 두 작업을 동시에 await — 두 Future 가 동시에 진행됨
    let (a, b) = tokio::join!(task(1, 100), task(2, 100));

    println!("결과: {a}, {b}");
    println!("총 소요: {:?}", start.elapsed());
}
```

### 예제 3 — `ex03_channel.rs` : tokio mpsc

```rust
// 22편 예제 3: tokio::sync::mpsc 로 비동기 메시지 채널
//
// 21편의 std::sync::mpsc 와 닮았지만, 송수신이 `.await` 가능해
// async 런타임 위에서 자연스럽게 동작합니다.

use tokio::sync::mpsc;
use tokio::task;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<String>(8); // 버퍼 8

    // 생산자 — 별도 비동기 task 로 spawn
    let producer = task::spawn(async move {
        for word in ["hello", "async", "rust"] {
            tx.send(word.to_string()).await.expect("받는 쪽 닫힘");
        }
        // tx 가 여기서 drop 됨 → rx 도 종료를 인지
    });

    // 소비자 — 메인 task 에서 받기
    while let Some(msg) = rx.recv().await {
        println!("받음: {msg}");
    }

    producer.await.expect("생산자 task 패닉");
    println!("완료");
}
```

## 자주 하는 실수

### Q. `async fn` 을 호출했는데 아무 일도 안 일어납니다.

A. `async fn` 의 반환값은 "할 일" 을 묶은 `Future` 입니다. `.await` 를 붙이거나 `tokio::spawn(...)` 으로 런타임에 맡겨야 실제로 실행됩니다.

### Q. `tokio::join!` 과 두 번의 `.await` 차이?

A. 두 번의 `.await` 는 순차 — 앞 작업이 끝나야 다음이 시작합니다. `join!` 은 두 작업을 **동시에** 진행시킵니다. I/O 대기처럼 외부를 기다리는 작업이 많을수록 차이가 큽니다.

### Q. `tokio::spawn` 한 task 가 안 끝나는 것 같아요.

A. main 이 끝나면 tokio 런타임도 함께 종료되어 spawn 한 task 들이 강제로 끊깁니다. 끝까지 기다리려면 `task.await` 로 명시적으로 join 하세요.

### Q. `std::sync::mpsc` 와 `tokio::sync::mpsc` 중 어느 쪽?

A. async 컨텍스트(`async fn` 안)에서 송수신이 일어난다면 `tokio::sync::mpsc`. 그렇지 않은 일반 스레드 간 채널이라면 `std::sync::mpsc`. async 컨텍스트에서 std 채널을 쓰면 스레드를 막아 다른 task 진행을 방해할 수 있습니다.

## 정리

- `async fn` 은 `Future` 를 돌려주고, `.await` 가 진행 트리거.
- 비동기 런타임이 필요하다 — 가장 흔한 선택이 tokio.
- `tokio::join!` 으로 여러 작업을 동시에, `tokio::spawn` 으로 백그라운드.
- 비동기 컨텍스트에서는 `tokio::sync::mpsc` 같은 비동기 채널을 사용.

## 직접 해 보기

`homework/` 폴더의 과제를 풀어 보세요. 정답은 `homework/answer/` 에 있습니다.

## 다음 단원

축하합니다 — Rust 입문 22편 완주! 🎉

이제 [The Rust Book](https://doc.rust-lang.org/book/) 의 후반부(스마트 포인터·매크로·고급 트레잇) 와 [Rust by Example](https://doc.rust-lang.org/rust-by-example/) 의 다양한 예제로 깊이를 더할 수 있습니다. 실전 영역으로는 웹 서버(axum, actix-web), CLI 도구(clap), 임베디드(embassy), 게임(bevy) 같은 생태계가 기다리고 있습니다. 즐거운 Rust 여행 되세요!
