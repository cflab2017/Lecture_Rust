// 6편 예제 3: 반환값으로 소유권 회수하기
//
// 함수에 String 을 넘기면 소유권이 이동합니다. 다시 쓰고 싶다면
// 반환값으로 돌려받거나, 다음 편에서 배울 "참조" 를 쓰면 됩니다.

fn main() {
    let s = String::from("hello");
    let s = grow(s); // s 를 넘기고 반환된 새 String 을 다시 s 에 묶음 (섀도잉)
    println!("결과: {s}");
}

fn grow(mut s: String) -> String {
    s.push_str(", world!");
    s // 소유권을 호출자에게 돌려줌
}
