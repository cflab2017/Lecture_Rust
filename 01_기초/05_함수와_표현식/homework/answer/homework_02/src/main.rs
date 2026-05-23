// 핵심 포인트:
// - 함수 두 개가 동일 변환의 양 방향을 각각 책임진다.
// - `///` 문서화 주석은 `cargo doc` 으로 HTML 문서를 만들 때 함께 수집된다.

/// 섭씨를 화씨로 변환합니다. `°F = °C * 9 / 5 + 32`
fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

/// 화씨를 섭씨로 변환합니다. `°C = (°F − 32) * 5 / 9`
fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn main() {
    let c = 25.0;
    let f = 98.6;

    println!("{:.1}°C = {:.1}°F", c, celsius_to_fahrenheit(c));
    println!("{:.1}°F = {:.1}°C", f, fahrenheit_to_celsius(f));
}
