// 16편 예제 2: Option 에도 ? 가 동작
//
// 함수 반환 타입이 Option<T> 일 때, ? 는 None 을 그대로 호출자에게 전파한다.

fn last_char_uppercase(s: &str) -> Option<char> {
    let last = s.chars().last()?;   // 빈 문자열이면 None 으로 즉시 반환
    last.to_uppercase().next()
}

fn main() {
    for s in ["hello", "rust", ""] {
        println!("{:?} → {:?}", s, last_char_uppercase(s));
    }
}
