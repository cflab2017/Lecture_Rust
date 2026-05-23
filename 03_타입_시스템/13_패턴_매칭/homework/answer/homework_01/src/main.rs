// 핵심 포인트:
// - `|` 로 여러 패턴을 한 분기로 묶을 수 있다.
// - 모든 variant 를 빠짐없이 적어 두면 새 variant 가 추가됐을 때 컴파일러가 알려 준다.

#[derive(Debug)]
enum Day {
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat,
    Sun,
}

fn kind(d: &Day) -> &'static str {
    match d {
        Day::Mon | Day::Tue | Day::Wed | Day::Thu | Day::Fri => "평일",
        Day::Sat | Day::Sun => "주말",
    }
}

fn main() {
    let week = [Day::Mon, Day::Tue, Day::Wed, Day::Thu, Day::Fri, Day::Sat, Day::Sun];
    for d in &week {
        println!("{:?} → {}", d, kind(d));
    }
}
