// 핵심 포인트:
// - `Option<&str>` 로 반환하면 "없을 수도 있다" 를 타입으로 표현 가능.
// - `split_whitespace().nth(n)` 은 곧 `Option<&str>` 을 돌려준다.

fn nth_word(s: &str, n: usize) -> Option<&str> {
    s.split_whitespace().nth(n)
}

fn main() {
    let text = "Rust is a systems language";

    println!("0번째 단어: {:?}", nth_word(text, 0));
    println!("1번째 단어: {:?}", nth_word(text, 1));
    println!("2번째 단어: {:?}", nth_word(text, 2));
    println!("10번째 단어: {:?}", nth_word(text, 10));
}
