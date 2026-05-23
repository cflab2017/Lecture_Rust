// 10편 예제 3: String(소유) ↔ &str(빌림) 변환

fn describe(s: &str) -> String {
    format!("입력 \"{s}\" 의 길이는 {} 바이트", s.len())
}

fn main() {
    let owned: String = String::from("자유");
    let lit: &str = "리터럴";

    // &String → &str 은 deref 로 자동
    println!("{}", describe(&owned));
    println!("{}", describe(lit));

    // &str → String (소유 사본 만들기)
    let copied: String = lit.to_string();
    let copied2: String = String::from(lit);
    println!("copied = {copied}, copied2 = {copied2}");

    // String → &str (슬라이스 만들기)
    let borrowed: &str = owned.as_str();
    println!("borrowed = {borrowed}");
}
