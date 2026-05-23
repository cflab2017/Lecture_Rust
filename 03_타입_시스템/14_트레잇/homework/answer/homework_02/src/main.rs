// 핵심 포인트:
// - 동적 크기 타입(dyn Trait)을 Vec 에 담으려면 한 단계 인다이렉션이 필요하다 → Box.
// - 구체 타입이 다른 인스턴스도 같은 인터페이스로 다룰 수 있다.

trait Speak {
    fn speak(&self) -> String;
}

struct Cat;
struct Cow;

impl Speak for Cat {
    fn speak(&self) -> String { String::from("야옹") }
}

impl Speak for Cow {
    fn speak(&self) -> String { String::from("음매") }
}

fn main() {
    let animals: Vec<Box<dyn Speak>> = vec![Box::new(Cat), Box::new(Cow)];
    for a in &animals {
        println!("{}", a.speak());
    }
}
