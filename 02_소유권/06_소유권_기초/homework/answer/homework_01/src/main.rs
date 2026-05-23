// 핵심 포인트:
// - String 을 함수에 넘기면 소유권이 이동한다.
// - 튜플로 (원본, 결과) 를 함께 반환하면 호출자가 다시 사용할 수 있다.
// - 다음 편에서 배울 참조(&String)를 쓰면 이런 번거로움이 사라진다.

fn len_of(s: String) -> (String, usize) {
    let len = s.len();
    (s, len) // 소유권을 호출자에게 돌려주면서 길이도 함께 반환
}

fn main() {
    let s = String::from("Rust");
    let (s, len) = len_of(s); // 섀도잉으로 같은 이름에 다시 묶음
    println!("{s} 의 길이는 {len} 입니다.");
}
