// 4편 예제 1: if·else·else if 와 if 를 표현식으로 사용하기

fn main() {
    let score = 78;

    // 전통적인 if/else if/else 사슬
    if score >= 90 {
        println!("A");
    } else if score >= 80 {
        println!("B");
    } else if score >= 70 {
        println!("C");
    } else {
        println!("F");
    }

    // Rust 의 if 는 표현식이라 값으로 사용할 수 있다.
    let label = if score >= 60 { "통과" } else { "재시험" };
    println!("결과: {label}");

    // 단, 모든 분기의 타입이 같아야 한다.
    // let bad = if true { 1 } else { "오류" }; // ❌ 타입 불일치
}
