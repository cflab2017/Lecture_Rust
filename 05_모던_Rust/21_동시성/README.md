# 21. 스레드·채널·Arc/Mutex

Rust 의 동시성은 "공포의 뇌관" 으로 불리던 데이터 레이스를 컴파일 단계에서 차단합니다. 비결은 6 ~ 9 편의 소유권·빌림 규칙이 동시성에도 그대로 적용되기 때문이죠 — 같은 데이터를 두 스레드가 동시에 가변 참조할 수 없으니 데이터 레이스가 원천 봉쇄됩니다. 이번 편에서는 표준 라이브러리만 사용해 스레드 생성·채널·공유 가변 상태를 익힙니다.

## 학습 목표

- `std::thread::spawn` 으로 새 스레드를 만들고 `join` 으로 기다린다.
- `mpsc::channel` 로 스레드 간 메시지를 안전하게 주고받는다.
- `Arc<Mutex<T>>` 패턴으로 공유 가변 상태를 다룬다.
- `move` 클로저가 캡처에 어떤 영향을 주는지 안다.

## 핵심 개념

### 1) `thread::spawn` 과 `JoinHandle`

```rust
use std::thread;

let handle = thread::spawn(|| {
    // 새 스레드 본문
});
handle.join().expect("자식 스레드 패닉");
```

- `spawn` 은 즉시 새 스레드를 시작하고 `JoinHandle<T>` 를 돌려준다.
- `JoinHandle::join()` 은 스레드가 끝나기를 기다리고 본문의 반환값(또는 패닉 페이로드)을 돌려준다.

### 2) `move` 클로저

클로저가 바깥 변수를 사용한다면, 기본은 빌림입니다. 그런데 스레드는 부모보다 오래 살 수 있어서 빌림 참조가 위험해질 수 있죠. 그래서 `move` 키워드로 **소유권을 클로저로 이동** 시킵니다.

```rust
let s = String::from("hi");
thread::spawn(move || { println!("{s}") }); // s 의 소유권을 클로저로
```

### 3) `mpsc::channel` — 메시지 전달

> "다수 생산자, 단일 소비자(multi-producer, single-consumer)"

```rust
let (tx, rx) = mpsc::channel();
tx.send(value).expect("받는 쪽 닫힘");
for msg in rx { /* ... */ }
```

송신자 `tx` 는 `clone()` 으로 복제해 여러 스레드에 나눠 줄 수 있고, 모든 송신자가 drop 되면 수신자의 이터레이션이 자연스럽게 종료됩니다.

### 4) `Arc<Mutex<T>>` — 공유 가변 상태

| 타입 | 의미 |
|------|------|
| `Arc<T>` | 원자적 참조 카운팅 — 여러 스레드가 같은 값을 공유 |
| `Mutex<T>` | 한 시점에 한 스레드만 데이터에 접근 가능하게 잠금 |
| `Arc<Mutex<T>>` | 두 가지 조합 — 공유 가변 |

```rust
let counter = Arc::new(Mutex::new(0));
let c = Arc::clone(&counter);
thread::spawn(move || {
    let mut num = c.lock().unwrap();
    *num += 1;
});
```

`lock()` 은 `LockResult<MutexGuard<T>>` 를 돌려주고, `MutexGuard` 가 스코프를 벗어나면 잠금이 풀립니다(RAII).

### 5) `Send` 와 `Sync`

표준 라이브러리의 두 마커 트레잇이 동시성 안전을 표현합니다.
- `T: Send` — 소유권을 다른 스레드로 옮길 수 있음.
- `T: Sync` — 여러 스레드가 동시에 `&T` 를 가질 수 있음.

`Arc`, `Mutex` 가 이들을 만족하므로 안전하게 공유 가능. `Rc` 는 `Send` 가 아니라서 멀티스레드에서 컴파일 거부.

## 예제로 보기

### 예제 1 — `ex01_thread.rs` : 스레드 + join

```rust
// 21편 예제 1: std::thread 로 스레드 생성과 join
//
// thread::spawn 은 새 스레드를 시작하고 JoinHandle 을 돌려줍니다.
// JoinHandle::join() 으로 스레드가 끝나기를 기다리며 결과를 받을 수 있습니다.

use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..=3 {
            println!("[자식] 카운트 {i}");
            thread::sleep(Duration::from_millis(20));
        }
        // 마지막 표현식이 join 결과가 됨
        "자식 완료"
    });

    for i in 1..=3 {
        println!("[메인] 카운트 {i}");
        thread::sleep(Duration::from_millis(20));
    }

    let result = handle.join().expect("자식 스레드 패닉");
    println!("자식 join 결과: {result}");
}
```

### 예제 2 — `ex02_channel.rs` : 채널

```rust
// 21편 예제 2: mpsc::channel 로 스레드 간 메시지 전달
//
// "다수 생산자, 단일 소비자(multi-producer, single-consumer)" 채널입니다.
// Tx 는 clone 가능, Rx 는 하나뿐.

use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel::<String>();

    // 생산자 두 개
    let tx2 = tx.clone();
    thread::spawn(move || {
        for word in ["a1", "a2", "a3"] {
            tx.send(format!("[A] {word}")).expect("채널 닫힘");
        }
    });
    thread::spawn(move || {
        for word in ["b1", "b2", "b3"] {
            tx2.send(format!("[B] {word}")).expect("채널 닫힘");
        }
    });

    // 소비자 — 두 송신자 모두 drop 되면 자연스럽게 종료
    for msg in rx {
        println!("받음: {msg}");
    }
}
```

### 예제 3 — `ex03_arc_mutex.rs` : Arc<Mutex<T>>

```rust
// 21편 예제 3: Arc<Mutex<T>> 로 공유 가변 상태
//
// Arc — Atomically Reference Counted, 여러 스레드가 같은 데이터를 공유.
// Mutex — 한 시점에 한 스레드만 잠금을 잡아 데이터에 접근.

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..5 {
        // Arc::clone 으로 참조 카운터만 증가 (싸다)
        let c = Arc::clone(&counter);
        let h = thread::spawn(move || {
            // lock() 은 LockResult — 패닉을 전달한 다른 스레드가 없다면 안전
            let mut num = c.lock().expect("뮤텍스 락 실패");
            *num += 1;
        });
        handles.push(h);
    }

    for h in handles {
        h.join().expect("자식 스레드 패닉");
    }

    println!("counter = {}", counter.lock().expect("뮤텍스 락 실패"));
}
```

## 자주 하는 실수

### Q. `move` 키워드를 빼면 컴파일이 안 됩니다.

A. 클로저가 바깥 값을 빌리는데, 그 빌림이 부모 스코프보다 오래 살 수 있어서 컴파일러가 막습니다. `move || { ... }` 로 소유권을 클로저로 옮기면 해결됩니다.

### Q. `Rc<RefCell<T>>` 는 안 되나요?

A. 단일 스레드에서는 가능하지만, 멀티스레드에서는 `Send`/`Sync` 가 아니라서 컴파일러가 막습니다. 멀티스레드 공유에는 `Arc<Mutex<T>>` (또는 `RwLock`) 를 쓰세요.

### Q. `lock().unwrap()` 이 panic 을 일으킬 수 있다는데?

A. 다른 스레드가 락을 잡은 채로 패닉하면 뮤텍스가 "poisoned" 상태가 됩니다. 그 뒤의 `lock()` 은 `Err(PoisonError)` 를 돌려줍니다. 입문 단계에서는 보통 `.expect()` 또는 `.unwrap()` 으로 충분합니다.

### Q. 채널을 닫으려면 어떻게?

A. 모든 `tx` (송신자)가 drop 되면 채널이 자동으로 닫혀 수신자의 이터레이션이 종료됩니다. 명시적으로 닫는 메서드는 없습니다.

## 정리

- `thread::spawn` + `JoinHandle::join` 이 기본 패턴.
- 스레드로 값을 옮기려면 `move` 클로저.
- `mpsc::channel` 로 메시지 전달, `Arc<Mutex<T>>` 로 공유 가변 상태.
- 빌림 규칙 + `Send`/`Sync` 가 데이터 레이스를 컴파일 단계에서 차단.

## 직접 해 보기

`homework/` 폴더의 과제를 풀어 보세요. 정답은 `homework/answer/` 에 있습니다.

## 다음 단원

[22. async/await + tokio 입문](../22_async_await/README.md) — Rust 의 비동기 모델과 가장 인기 있는 런타임 tokio 를 입문합니다.
