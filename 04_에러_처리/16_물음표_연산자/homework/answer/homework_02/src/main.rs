// 핵심 포인트:
// - `find` 는 조건을 만족하는 첫 원소를 Option 으로 돌려준다.
// - `?` 는 Option 함수에서도 동작하지만, 본 예처럼 그대로 반환해도 짧다.

fn first_capital(s: &str) -> Option<char> {
    s.chars().find(|c| c.is_ascii_uppercase())
}

fn main() {
    for s in ["hello World", "rust", ""] {
        println!("{:?} → {:?}", s, first_capital(s));
    }
}
