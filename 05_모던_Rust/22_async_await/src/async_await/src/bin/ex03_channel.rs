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
