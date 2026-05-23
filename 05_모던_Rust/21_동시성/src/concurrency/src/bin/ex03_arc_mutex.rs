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
