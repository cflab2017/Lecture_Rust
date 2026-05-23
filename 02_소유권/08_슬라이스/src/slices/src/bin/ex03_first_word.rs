// 8편 예제 3: first_word — 슬라이스의 고전적 예시
//
// 문자열에서 첫 공백 직전까지의 슬라이스를 돌려준다.
// 인덱스(usize) 만 돌려주는 것보다 슬라이스가 안전합니다.

fn first_word(s: &str) -> &str {
    for (i, &b) in s.as_bytes().iter().enumerate() {
        if b == b' ' {
            return &s[..i];
        }
    }
    s
}

fn main() {
    let sentence = String::from("Rust 는 시스템 언어입니다");
    let word = first_word(&sentence);
    println!("첫 단어: {word}");

    // 문자열 리터럴에도 그대로 적용 가능
    let lit = "Hello World";
    println!("첫 단어(lit): {}", first_word(lit));
}
