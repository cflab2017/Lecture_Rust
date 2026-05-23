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
