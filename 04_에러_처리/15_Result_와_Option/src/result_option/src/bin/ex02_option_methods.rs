// 15편 예제 2: Option 메서드 모음
//
// map / and_then / unwrap_or / unwrap_or_else / ok_or

fn half(n: i32) -> Option<i32> {
    if n % 2 == 0 { Some(n / 2) } else { None }
}

fn main() {
    let v = Some(10);

    // map — Some 안의 값을 변환
    let plus_one = v.map(|x| x + 1);
    println!("map: {:?}", plus_one);

    // and_then — Some 일 때 다른 Option 반환 (체이닝)
    let chained = v.and_then(half).and_then(half);
    println!("and_then 두 번: {:?}", chained);

    // unwrap_or — None 일 때 기본값
    let backup = None::<i32>.unwrap_or(-1);
    println!("unwrap_or(-1) = {backup}");

    // ok_or — Option → Result 로 변환 (에러 메시지 추가)
    let r: Result<i32, &str> = Some(3).ok_or("값 없음");
    println!("ok_or = {:?}", r);
}
