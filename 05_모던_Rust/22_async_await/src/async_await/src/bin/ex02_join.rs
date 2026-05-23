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
