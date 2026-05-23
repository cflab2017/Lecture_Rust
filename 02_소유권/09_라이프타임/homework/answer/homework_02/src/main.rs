// 핵심 포인트:
// - 구조체가 참조 필드를 가지므로 `<'a>` 라이프타임 파라미터가 필요하다.
// - 문자열의 "문자 단위" 자르기는 `chars().take(N)` + `collect::<String>()` 이 안전.

struct Note<'a> {
    title: &'a str,
    body: &'a str,
}

impl<'a> Note<'a> {
    fn summary(&self) -> String {
        let n = self.body.chars().count();
        if n <= 20 {
            format!("{}: {}", self.title, self.body)
        } else {
            let head: String = self.body.chars().take(20).collect();
            format!("{}: {}…", self.title, head)
        }
    }
}

fn main() {
    let short = Note { title: "오늘의 메모", body: "짧은 본문." };
    let long = Note {
        title: "긴 메모",
        body: "This is a longer body for testing summary cutoff.",
    };

    println!("{}", short.summary());
    println!("{}", long.summary());
}
