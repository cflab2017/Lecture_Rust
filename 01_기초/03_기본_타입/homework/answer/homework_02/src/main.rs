// 핵심 포인트:
// - `iter().sum()` 은 컬렉션의 모든 원소를 합산한다.
// - 결과 타입이 모호하므로 `let sum: i32 = ...;` 처럼 좌변에 타입을 적어 준다.
// - 평균은 실수 계산 — `sum as f64 / len as f64`.

fn main() {
    let scores = [85, 90, 78, 92, 88, 70];

    let sum: i32 = scores.iter().sum();
    let avg = sum as f64 / scores.len() as f64;

    println!("점수: {:?}", scores);
    println!("총합: {sum}");
    println!("평균: {:.2}", avg);
}
