// 핵심 포인트:
// - `&str` 로 받으면 String / 문자열 리터럴 양쪽 모두에서 호출 가능하다.
// - `&text` 는 자동으로 `&String → &str` 로 디레퍼런스(Deref) 변환된다.

fn word_count(s: &str) -> usize {
    s.split_whitespace().count()
}

fn main() {
    let text = String::from("hello rust borrowing checker");
    let n = word_count(&text); // 소유권 이동 없음
    println!("text = {text}");
    println!("단어 수 = {n}");
}
