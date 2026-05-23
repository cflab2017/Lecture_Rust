// 7편 예제 1: 공유 참조(&T) 로 값을 "빌려" 보기
//
// 참조를 받으면 소유권이 이동하지 않으므로, 호출자는 변수를 계속 쓸 수 있다.

fn len_of(s: &String) -> usize {
    s.len()
} // 여기서 s 는 빌린 참조라 drop 되지 않음

fn main() {
    let s = String::from("Hello, Rust!");
    let n = len_of(&s); // 참조 만들기: `&s`
    println!("\"{s}\" 의 길이는 {n}");

    // 참조는 여러 개를 동시에 가질 수 있다(불변일 때).
    let r1 = &s;
    let r2 = &s;
    println!("같은 값을 두 참조로: {r1} / {r2}");
}
