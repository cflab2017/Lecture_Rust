// 13편 예제 1: match — 패턴 분기, 바인딩, 와일드카드, 가드

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(String), // 어느 주의 쿼터인지
}

fn describe(c: &Coin) -> String {
    match c {
        Coin::Penny => String::from("1 센트"),
        Coin::Nickel => String::from("5 센트"),
        Coin::Dime => String::from("10 센트"),
        // variant 의 내부 데이터를 바인딩
        Coin::Quarter(state) => format!("25 센트 ({state} 주)"),
    }
}

fn grade(score: i32) -> char {
    // _ 와일드카드 + 가드(if 조건)
    match score {
        s if s >= 90 => 'A',
        s if s >= 80 => 'B',
        s if s >= 70 => 'C',
        _ => 'F',
    }
}

fn main() {
    println!("{}", describe(&Coin::Penny));
    println!("{}", describe(&Coin::Quarter(String::from("Alaska"))));

    for s in [95, 82, 73, 50] {
        println!("{s} → {}", grade(s));
    }
}
