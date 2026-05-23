// 핵심 포인트:
// - `if let Some(n) = v` 는 v 가 Some 인 경우에만 본문을 실행한다.
// - `iter().flatten()` 은 Option<T> 의 시퀀스에서 None 을 자동으로 걸러 준다.

fn main() {
    let values: Vec<Option<i32>> = vec![Some(1), None, Some(3), Some(5), None, Some(7)];

    let mut total = 0;
    for v in &values {
        if let Some(n) = v {
            total += n;
        }
    }
    println!("Some 값들의 합 = {total}");

    // 한 줄 풀이도 가능 (참고)
    // let total: i32 = values.iter().flatten().sum();
}
