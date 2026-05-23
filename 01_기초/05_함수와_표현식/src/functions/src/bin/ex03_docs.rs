// 5편 예제 3: 문서화 주석(///)
// 함수 위에 `///` 를 적어두면 `cargo doc` 으로 HTML 문서를 생성할 수 있습니다.

/// 두 정수의 합을 계산합니다.
///
/// # 예시
/// ```
/// // (doctest 예시는 라이브러리 크레잇에서 실행됩니다.)
/// let s = 1 + 2;
/// assert_eq!(s, 3);
/// ```
fn sum(a: i32, b: i32) -> i32 {
    a + b
}

/// 섭씨를 화씨로 변환합니다.
///
/// 공식: `°F = °C * 9 / 5 + 32`
fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

fn main() {
    println!("sum(2, 3) = {}", sum(2, 3));
    println!("0°C  -> {:.1}°F", celsius_to_fahrenheit(0.0));
    println!("100°C -> {:.1}°F", celsius_to_fahrenheit(100.0));

    println!();
    println!("문서 생성: `cargo doc --open` 으로 브라우저에서 확인할 수 있습니다.");
}
