// 9편 예제 3: 라이프타임 생략 규칙 (Lifetime Elision)
//
// 자주 등장하는 패턴은 컴파일러가 라이프타임을 자동으로 채워 줍니다.
// 규칙:
// 1) 각 참조 입력은 자기만의 라이프타임을 부여받는다.
// 2) 입력이 정확히 하나면 그 라이프타임을 출력에 적용한다.
// 3) 메서드의 첫 인수가 &self / &mut self 이면 그 라이프타임을 출력에 적용한다.

// (1) 생략 가능 — 입력 참조 1개 → 출력에 그대로 적용
fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}

// (2) 생략 불가 — 입력이 두 개라 어느 쪽 라이프타임을 출력에 적용할지 모름
//     명시가 필요하다.
fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

// (3) 메서드 — &self 라이프타임이 자동 적용
struct Holder<'a> { value: &'a str }

impl<'a> Holder<'a> {
    fn value(&self) -> &str { self.value } // 라이프타임 생략됨
}

fn main() {
    let s = String::from("Rust language is fast");
    println!("first_word = {}", first_word(&s));
    println!("longer     = {}", longer("aaa", "bb"));

    let h = Holder { value: &s };
    println!("holder     = {}", h.value());
}
