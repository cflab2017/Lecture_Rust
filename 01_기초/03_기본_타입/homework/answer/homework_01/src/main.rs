// 핵심 포인트:
// - 정수와 실수는 자동 변환되지 않으므로 `as f64` 캐스팅이 필요하다.
// - 소수 둘째 자리 포맷팅은 `{:.2}`.

fn main() {
    let a = 17;
    let b = 5;

    let q = a / b;
    let r = a % b;
    println!("{a} ÷ {b} = {q}, 나머지 {r}");

    let f = a as f64 / b as f64;
    println!("{:.1} / {:.1} = {:.2}", a as f64, b as f64, f);
}
