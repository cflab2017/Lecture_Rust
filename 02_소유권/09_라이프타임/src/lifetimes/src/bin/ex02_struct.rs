// 9편 예제 2: 참조를 가지는 구조체와 라이프타임
//
// 필드에 참조를 두는 구조체는 라이프타임 파라미터가 필요합니다.
// 구조체 인스턴스는 그 필드가 가리키는 값보다 오래 살 수 없습니다.

struct Excerpt<'a> {
    part: &'a str,
}

impl<'a> Excerpt<'a> {
    fn announce(&self, prefix: &str) -> &str {
        println!("[{prefix}] 발췌 미리보기: {}", self.part);
        self.part
    }
}

fn main() {
    let article = String::from("Rust 의 라이프타임은 참조가 유효한 기간을 의미합니다.");
    let first_sentence = article.split('.').next().expect("문장 없음");

    let ex = Excerpt { part: first_sentence };
    let kept = ex.announce("INFO");

    println!("kept = {kept}");
    // article 이 살아 있는 동안 ex.part / kept 모두 유효
}
