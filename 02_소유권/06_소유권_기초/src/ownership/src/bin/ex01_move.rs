// 6편 예제 1: String 의 소유권 이동(move)
//
// 힙에 데이터를 가지는 타입(String, Vec, Box 등) 은 대입이나 함수 호출에서
// 값이 "이동" 됩니다. 이동된 변수는 더 이상 사용할 수 없습니다.

fn main() {
    let s1 = String::from("안녕"); // s1 이 힙 데이터의 소유자
    let s2 = s1;                    // 소유권이 s2 로 이동
    // println!("{s1}");            // ❌ 컴파일 에러: s1 은 더 이상 유효하지 않음
    println!("s2 = {s2}");

    let s3 = String::from("Rust");
    take(s3);                       // 함수에 넘기면 인수로 소유권 이동
    // println!("{s3}");            // ❌ s3 도 더 이상 유효하지 않음

    println!("스코프가 끝나면 소유자가 drop 되어 메모리가 해제됩니다.");
}

fn take(s: String) {
    println!("take 가 받은 값: {s}");
    // 함수가 끝나면서 s 가 drop 됨
}
