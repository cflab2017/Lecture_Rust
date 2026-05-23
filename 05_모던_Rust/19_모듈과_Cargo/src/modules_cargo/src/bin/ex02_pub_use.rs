// 19편 예제 2: use 로 경로 줄이기
//
// 자주 쓰는 항목은 `use` 로 가져와 짧은 이름만 쓰면 가독성이 좋아집니다.

mod text {
    pub fn shout(s: &str) -> String {
        s.to_uppercase() + "!"
    }

    pub fn whisper(s: &str) -> String {
        format!("({})", s.to_lowercase())
    }
}

// 모듈 함수들을 현재 스코프로 가져오기
use text::{shout, whisper};

// 자기 모듈을 별칭으로 가져올 수도 있다.
use text as tt;

fn main() {
    println!("{}", shout("hello"));
    println!("{}", whisper("Quiet"));

    // 별칭으로도 호출
    println!("{}", tt::shout("hi"));
}
