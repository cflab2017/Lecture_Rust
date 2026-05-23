// 핵심 포인트:
// - `parse` 는 결과가 일반화되어 있어 `::<i32>` 로 타입을 명시한다.
// - 실패 시 기본값 회복은 `unwrap_or(default)` 한 줄.

fn parse_or_default(s: &str) -> i32 {
    s.trim().parse::<i32>().unwrap_or(0)
}

fn main() {
    for s in ["42", "abc", "  -7  "] {
        println!("{}", parse_or_default(s));
    }
}
