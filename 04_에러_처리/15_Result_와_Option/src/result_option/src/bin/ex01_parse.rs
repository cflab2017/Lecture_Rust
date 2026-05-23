// 15편 예제 1: str.parse 의 Result 처리
//
// `parse::<i32>()` 는 `Result<i32, ParseIntError>` 를 돌려준다.
// 성공·실패를 호출자가 명시적으로 처리해야 한다.

fn main() {
    let inputs = ["42", "오류", "  7  ", "-3"];

    for s in inputs {
        // 공백 제거 후 parse
        let parsed = s.trim().parse::<i32>();

        match parsed {
            Ok(n) => println!("'{s}' → {n}"),
            Err(e) => println!("'{s}' → 실패: {e}"),
        }
    }

    // unwrap_or: 실패 시 기본값
    let n = "abc".parse::<i32>().unwrap_or(0);
    println!("unwrap_or 결과 = {n}");
}
