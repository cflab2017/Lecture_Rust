// 핵심 포인트:
// - "결과가 없을 수도 있음" 은 Option 으로 표현하면 호출자가 강제로 분기하게 된다.
// - 부동소수 비교는 안전성을 위해 `==` 직접 비교를 피하는 게 일반적이지만,
//   0.0 같이 정확한 값은 == 비교가 안전하다.

fn safe_div(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}

fn main() {
    let cases = [(10.0, 2.0), (7.0, 0.0)];
    for (a, b) in cases.iter() {
        match safe_div(*a, *b) {
            Some(v) => println!("결과: {:.2}", v),
            None => println!("0으로 나눌 수 없음"),
        }
    }
}
