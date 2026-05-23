// 10편 예제 1: 문자열 결합 방법
//
// push_str / push / + 연산자 / format! 매크로

fn main() {
    // 가변 String 에 직접 덧붙이기
    let mut greeting = String::from("Hello");
    greeting.push_str(", ");      // 문자열 슬라이스를 추가
    greeting.push('R');           // 한 글자(char)
    greeting.push_str("ust!");
    println!("push 계열: {greeting}");

    // + 연산자 — 좌변은 String 소유권, 우변은 &str
    let a = String::from("안녕, ");
    let b = String::from("세계!");
    let ab = a + &b; // a 는 이동, b 는 빌림
    println!("+ 연산자: {ab}");

    // format! — 새 String 을 만들고 어떤 인수도 소유권을 가져가지 않음
    let name = String::from("Rust");
    let lang = String::from("Korean");
    let sentence = format!("{} 강의를 {} 로 듣고 있어요.", name, lang);
    println!("format!: {sentence}");
    println!("(원본 그대로 사용 가능) name={name}, lang={lang}");
}
