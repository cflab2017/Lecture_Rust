// 핵심 포인트:
// - 각 스레드는 자기 구간의 합을 계산한 뒤 마지막 표현식으로 반환한다.
// - JoinHandle::join() 의 결과는 Result — unwrap 해서 값을 회수한다.

use std::thread;

fn main() {
    let ranges = [(1i32, 25), (26, 50), (51, 75), (76, 100)];
    let mut handles = vec![];
    for (start, end) in ranges {
        let h = thread::spawn(move || (start..=end).sum::<i32>());
        handles.push(h);
    }

    let mut total = 0;
    for h in handles {
        total += h.join().expect("자식 스레드 패닉");
    }

    println!("1..=100 의 총합 = {total}");
}
