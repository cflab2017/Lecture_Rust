// 8편 예제 1: 문자열 슬라이스 &str
//
// String 의 일부분을 빌리는 타입이 &str 입니다.
// 문자열 리터럴 "abc" 도 사실은 &'static str 입니다.

fn main() {
    let s = String::from("Hello, World!");

    // 범위는 [시작..끝). 끝은 제외.
    let hello: &str = &s[0..5];
    let world: &str = &s[7..12];
    let all: &str = &s[..]; // 전체

    println!("hello = {hello}");
    println!("world = {world}");
    println!("all   = {all}");

    // &str 받는 함수는 String 도 받을 수 있다 (deref 변환)
    print_len("Rust");
    print_len(&s);
}

fn print_len(s: &str) {
    println!("\"{s}\" 의 바이트 길이: {}", s.len());
}
