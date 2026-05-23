// 핵심 포인트:
// - `tokio::join!` 은 여러 Future 를 동시에 진행시켜 한 번에 결과를 반환한다.
// - 50ms 짜리 작업 3개가 순차 await 면 150ms, join! 이면 약 50ms.

use tokio::time::{sleep, Duration};

async fn square(n: u64) -> u64 {
    sleep(Duration::from_millis(50)).await;
    n * n
}

#[tokio::main]
async fn main() {
    let (a, b, c) = tokio::join!(square(3), square(4), square(5));
    println!("3² + 4² + 5² = {}", a + b + c);
}
