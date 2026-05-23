// 18편 예제 3: 두 종류 외부 에러를 한 커스텀 에러로 통합
//
// 실전에서는 IO 와 파싱처럼 다른 출처의 에러가 한 함수에서 만납니다.
// 같은 커스텀 에러로 묶어 두면 호출자가 처리하기 편합니다.

use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug, Error)]
enum AppError {
    #[error("IO 실패: {0}")]
    Io(#[from] std::io::Error),
    #[error("파싱 실패: {0}")]
    Parse(#[from] ParseIntError),
}

fn read_and_double(path: &str) -> Result<i32, AppError> {
    let s = std::fs::read_to_string(path)?;   // io::Error → AppError
    let n: i32 = s.trim().parse()?;           // ParseIntError → AppError
    Ok(n * 2)
}

fn main() {
    let path = std::env::temp_dir().join("lecture18_ex03.txt");
    std::fs::write(&path, "10").expect("임시 파일 작성");
    let path_str = path.to_str().expect("UTF-8 경로");

    match read_and_double(path_str) {
        Ok(v) => println!("결과 = {v}"),
        Err(e) => println!("실패: {e}"),
    }

    // 일부러 잘못된 내용을 써서 파싱 실패도 보여 준다.
    std::fs::write(&path, "not-a-number").expect("임시 파일 재작성");
    match read_and_double(path_str) {
        Ok(v) => println!("결과 = {v}"),
        Err(e) => println!("실패: {e}"),
    }
}
