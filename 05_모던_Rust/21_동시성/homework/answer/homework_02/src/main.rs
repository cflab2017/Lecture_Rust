// 핵심 포인트:
// - 송신자 tx 를 clone() 해서 여러 스레드가 같은 채널에 보낼 수 있다.
// - 모든 송신자가 drop 되면 수신자의 이터레이션이 자연스럽게 종료된다.

use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel::<String>();
    let tx2 = tx.clone();

    thread::spawn(move || {
        for w in ["hello", "world"] {
            tx.send(w.to_string()).expect("채널 닫힘");
        }
    });
    thread::spawn(move || {
        for w in ["from", "rust"] {
            tx2.send(w.to_string()).expect("채널 닫힘");
        }
    });

    let words: Vec<String> = rx.iter().collect();
    println!("받은 단어 수: {}", words.len());
}
