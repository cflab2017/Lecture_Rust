# 과제 - 22. async/await + tokio

## 문제 1 — async sleep + 출력
- 파일명: `homework_01/src/main.rs`
- 핵심 개념: `async fn`, `.await`, `tokio::time::sleep`

### 요구사항
- 함수 `delayed_print(name: &str, ms: u64)` 를 async 로 정의한다.
  - `ms` 밀리초 동안 `sleep` 한 뒤 `"{name} 완료 ({ms}ms)"` 를 출력.
- main 에서 차례로 두 번 await 한다.
  - `delayed_print("작업1", 50)`
  - `delayed_print("작업2", 30)`

### 예상 출력
```
작업1 완료 (50ms)
작업2 완료 (30ms)
```

### 힌트
- `use tokio::time::{sleep, Duration};`
- `#[tokio::main]` 매크로로 main 을 async 로.

## 문제 2 — `tokio::join!` 으로 동시 실행
- 파일명: `homework_02/src/main.rs`
- 핵심 개념: `tokio::join!`, 비동기 동시성

### 요구사항
- async 함수 `square(n: u64) -> u64` 가 50ms 후에 `n*n` 을 돌려준다.
- main 에서 `tokio::join!(square(3), square(4), square(5))` 로 세 작업을 동시에 진행.
- 합을 출력.

### 예상 출력
```
3² + 4² + 5² = 50
```

### 힌트
- `sleep(Duration::from_millis(50)).await; n * n`.
- `let (a, b, c) = tokio::join!(...);`.

## 정답 확인

직접 풀어 본 후 [`answer/`](./answer/) 폴더의 정답과 비교해 보세요.
