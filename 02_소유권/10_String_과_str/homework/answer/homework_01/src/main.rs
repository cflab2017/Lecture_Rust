// 핵심 포인트:
// - 바이트가 아닌 문자(char) 단위로 뒤집어야 멀티바이트 문자도 안전하다.
// - `chars().rev()` 는 문자 단위 역방향 이터레이터를 만든다.

fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

fn main() {
    println!("reverse(\"Hello\") = {}", reverse("Hello"));
    println!("reverse(\"Rust 한글\") = {}", reverse("Rust 한글"));
}
