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
