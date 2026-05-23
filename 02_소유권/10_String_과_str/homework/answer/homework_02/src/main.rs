// 핵심 포인트:
// - `matches!` 매크로는 패턴 매칭을 한 줄에 표현할 수 있는 편리한 도구.
// - 대소문자를 통일한 뒤 비교하면 분기가 간결해진다.

fn count_vowels(s: &str) -> usize {
    s.chars()
        .filter(|c| matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u'))
        .count()
}

fn main() {
    let input = "Programming in Rust";
    println!("\"{input}\" 의 모음 개수 = {}", count_vowels(input));
}
