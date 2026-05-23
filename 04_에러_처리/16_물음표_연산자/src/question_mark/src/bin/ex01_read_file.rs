// 16편 예제 1: 파일 읽고 숫자 파싱 — ? 로 두 종류 에러 자연스럽게 전파
//
// `?` 는 Result/Option 이 Err/None 일 때 함수 자체를 즉시 종료하며
// 에러를 호출자에게 돌려준다. 두 다른 에러 타입이 모두 `Box<dyn Error>` 로
// 자동 변환되어 한 줄에 묶인다.

use std::error::Error;
use std::fs;

fn read_first_number(path: &str) -> Result<i32, Box<dyn Error>> {
    let content = fs::read_to_string(path)?; // io::Error
    let n: i32 = content.trim().parse()?;    // ParseIntError
    Ok(n)
}

fn main() -> Result<(), Box<dyn Error>> {
    // 임시 경로에 파일을 만들어 두고 읽는다.
    let path = std::env::temp_dir().join("lecture16_ex01.txt");
    fs::write(&path, "42")?;

    let n = read_first_number(path.to_str().expect("UTF-8 경로"))?;
    println!("읽은 숫자: {n}");

    Ok(())
}
