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
