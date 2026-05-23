// 16편 예제 3: ? 와 From 자동 변환
//
// `?` 는 단순히 에러를 그대로 돌려보내는 게 아니라, From 트레잇이 구현되어
// 있으면 호출자 쪽 에러 타입으로 **자동 변환** 합니다.

use std::num::ParseIntError;

#[derive(Debug)]
enum MyError {
    Parse(ParseIntError),
    OutOfRange(i32),
}

// 표준 라이브러리 에러를 우리 enum 으로 감싸기 위한 From 구현
impl From<ParseIntError> for MyError {
    fn from(e: ParseIntError) -> Self {
        MyError::Parse(e)
    }
}

fn parse_age(s: &str) -> Result<u8, MyError> {
    let n: i32 = s.parse()?; // ParseIntError → MyError 자동 변환
    if !(0..=150).contains(&n) {
        return Err(MyError::OutOfRange(n));
    }
    Ok(n as u8)
}

fn main() {
    for s in ["25", "0", "abc", "200", "-3"] {
        println!("{s:?} → {:?}", parse_age(s));
    }
}
