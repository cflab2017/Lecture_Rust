// 핵심 포인트:
// - 트레잇의 기본 메서드는 구현체가 그대로 두면 자동 사용된다.
// - 다른 구현체에서는 같은 메서드를 자유롭게 오버라이드할 수 있다.

trait Greet {
    fn name(&self) -> &str;
    fn hello(&self) -> String {
        format!("안녕, {}!", self.name())
    }
}

struct Person { name: String }
struct Dog { name: String }

impl Greet for Person {
    fn name(&self) -> &str { &self.name }
    // hello 는 기본 구현 사용
}

impl Greet for Dog {
    fn name(&self) -> &str { &self.name }
    fn hello(&self) -> String { format!("{}: 멍멍!", self.name) }
}

fn main() {
    let p = Person { name: String::from("지수") };
    let d = Dog { name: String::from("초코") };

    println!("{}", p.hello());
    println!("{}", d.hello());
}
