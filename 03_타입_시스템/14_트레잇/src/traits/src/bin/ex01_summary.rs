// 14편 예제 1: 트레잇 정의·구현·기본 메서드
//
// 트레잇은 다른 언어의 "인터페이스" 와 비슷하지만, 기본 메서드를 가질 수 있어
// 부분 구현을 강제하지 않고도 공통 동작을 제공할 수 있습니다.

trait Summary {
    // 구현이 비어 있는 메서드 — 구현체가 반드시 정의해야 함
    fn title(&self) -> String;

    // 기본 메서드 — 구현체가 오버라이드하지 않으면 이 본문이 쓰임
    fn summarize(&self) -> String {
        format!("({}) 의 요약입니다.", self.title())
    }
}

struct Article {
    headline: String,
    body: String,
}

impl Summary for Article {
    fn title(&self) -> String {
        self.headline.clone()
    }
    // summarize 는 기본 구현 사용

    // body 까지 같이 보고 싶다면 직접 override 도 가능
    //fn summarize(&self) -> String {
    //    format!("[{}] {} ...", self.headline, &self.body[..self.body.len().min(20)])
    //}
}

struct Tweet {
    user: String,
    text: String,
}

impl Summary for Tweet {
    fn title(&self) -> String {
        format!("@{}", self.user)
    }
    fn summarize(&self) -> String {
        format!("@{}: {}", self.user, self.text)
    }
}

fn main() {
    let a = Article {
        headline: String::from("Rust 1.0 출시"),
        body: String::from("Rust 가 안정판에 도달했습니다."),
    };
    let t = Tweet {
        user: String::from("rustlang"),
        text: String::from("hello from rustlang"),
    };
    println!("{}", a.summarize());
    println!("{}", t.summarize());
}
