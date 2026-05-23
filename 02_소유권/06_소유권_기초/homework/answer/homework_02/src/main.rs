// 핵심 포인트:
// - i32 는 Copy 타입이라 함수에 넘겨도 원본이 무효화되지 않는다.
// - 함수 안에서 받은 값은 사실상 "복사본" 이다.

fn print_squared(n: i32) {
    println!("제곱: {}", n * n);
}

fn main() {
    let n = 42;
    print_squared(n);
    println!("원본: {n}"); // 여전히 사용 가능
}
