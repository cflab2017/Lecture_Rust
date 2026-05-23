// 13편 예제 2: if let — 한 가지 variant 만 다룰 때 간결한 문법
//
// match 와 비교해 "나머지는 무시" 가 자연스러울 때 적합합니다.

fn main() {
    let maybe = Some(42);

    // match 로 처리
    match maybe {
        Some(n) => println!("(match) 값 = {n}"),
        None => {}
    }

    // 같은 동작을 if let 으로 짧게
    if let Some(n) = maybe {
        println!("(if let) 값 = {n}");
    }

    // else 도 가능
    let nothing: Option<i32> = None;
    if let Some(n) = nothing {
        println!("값 = {n}");
    } else {
        println!("(if let else) 값 없음");
    }
}
