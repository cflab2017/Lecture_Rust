// 핵심 포인트:
// - `async fn` 은 즉시 실행되지 않으므로 `.await` 가 필요하다.
// - `#[tokio::main]` 매크로 한 줄로 동기 main 을 async main 으로 바꿀 수 있다.

use tokio::time::{sleep, Duration};

async fn delayed_print(name: &str, ms: u64) {
    sleep(Duration::from_millis(ms)).await;
    println!("{name} 완료 ({ms}ms)");
}

#[tokio::main]
async fn main() {
    delayed_print("작업1", 50).await;
    delayed_print("작업2", 30).await;
}
