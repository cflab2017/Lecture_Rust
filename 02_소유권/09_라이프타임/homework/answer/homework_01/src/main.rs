// 핵심 포인트:
// - 두 입력 참조 중 하나를 돌려주므로 라이프타임 명시가 필요하다.
// - 둘 다 `'a` 로 묶어 "두 입력 모두 살아 있는 동안" 이라는 의미를 표현한다.

fn shortest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() <= b.len() { a } else { b }
}

fn main() {
    println!("짧은 것: {}", shortest("hello", "hi"));
    println!("짧은 것: {}", shortest("같음", "같음"));
}
