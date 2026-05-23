// 핵심 포인트:
// - 빈 슬라이스에 대해 나눗셈을 시도하지 않도록 `is_empty()` 분기를 먼저 둔다.
// - `sum::<f64>()` 는 결과 타입을 명시해야 추론이 가능하다.

fn average(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    values.iter().sum::<f64>() / values.len() as f64
}

fn main() {
    let xs = [10.0, 20.0, 30.0, 40.0];
    println!("{:?} 의 평균 = {:.2}", xs, average(&xs));

    let empty: [f64; 0] = [];
    println!("빈 슬라이스의 평균 = {:.2}", average(&empty));
}
